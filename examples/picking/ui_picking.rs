//! Demonstrates the use of picking in Bevy's UI. Also shows use of one-shot systems to spawn
//! boilerplate UI on-demand.
//!
//! This example displays a simple inventory. Items can be dragged and dropped within the
//! inventory.

use bevy::{ecs::system::RunSystemOnce, prelude::*, window::PrimaryWindow};

fn main() {
    App::new()
        .init_resource::<DragDetails>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, (setup, spawn_text))
        .add_systems(Update, drag_inventory_items)
        .run();
}

#[derive(Resource, Copy, Clone, Default)]
struct DragDetails {
    pub drag_origin: Option<Entity>,
    pub drop_target: Option<Entity>,
}

/// Marker component for the root inventory UI `Node`.
#[derive(Component)]
struct Inventory;

/// Marker component for an individual slot within the Inventory.
#[derive(Component)]
struct InventorySlot;

/// Marker component for entities which can be dragged.
#[derive(Component)]
struct Dragable;

/// Marker component for entities which are currently being dragged, and should follow the pointer.
#[derive(Component)]
struct Dragging;

/// Set up some UI to manipulate.
///
/// Systems that accept a &mut World argument implement the `Command` trait. We use this rather
/// than a system accepting `Commands`, because we want access to one-shot systems below.
fn setup(world: &mut World) {
    // We need a simple camera to display our UI.
    world.spawn((Name::new("Camera"), Camera2d));

    world.spawn((
        Name::new("Inventory"),
        Inventory,
        // This first node acts like a container. You can think of it as the box in which
        // inventory slots are arranged. It's useful to remember that picking events can
        // "bubble" up through layers of UI, so if required this parent node can act on events
        // that are also received by its child nodes. You could imagine using this for a subtle
        // color change or border highlight.
        Node {
            align_items: AlignItems::Center,
            align_self: AlignSelf::Center,
            justify_content: JustifyContent::Center,
            justify_self: JustifySelf::Center,
            padding: UiRect::all(Val::Px(10.)),
            ..default()
        },
        BackgroundColor(Color::WHITE.with_alpha(0.1)),
    ));

    // To reduce boilerplate, let's spawn the inventory slots using one-shot systems. The slots
    // are each identical, and the UI Layout engine takes care of arranging them for us in the
    // above container. The `.unwrap()` is for simplicity in this example, whereas you might
    // use `.expect()`.
    world.run_system_once(spawn_inventory_slot).unwrap();
    world.run_system_once(spawn_inventory_slot).unwrap();
    world.run_system_once(spawn_inventory_slot).unwrap();
    world.run_system_once(spawn_inventory_slot).unwrap();

    // Now we need to add some items to the inventory.
    world.run_system_once(add_items_to_inventory).unwrap();
}

/// Spawn individual inventory slots, using the `Inventory` root UI node as its parent.
fn spawn_inventory_slot(mut commands: Commands, inventory: Single<Entity, With<Inventory>>) {
    commands.entity(*inventory).with_children(|parent| {
        parent
            .spawn((
                Name::new("Inventory Slot"),
                Node {
                    margin: UiRect::all(Val::Px(5.)),
                    height: Val::Px(50.),
                    width: Val::Px(50.),
                    padding: UiRect::all(Val::Px(5.)),
                    ..default()
                },
                InventorySlot,
                BackgroundColor(Color::WHITE.with_alpha(0.5)),
            ))
            // Update the `DragDetails` as the UI `Node`s receive picking events.
            .observe(set_drop_target())
            .observe(clear_drop_target());
    });
}

/// This observer detects the beginning of a drag-and-drop motion and takes care of tasks such as
/// detaching the target from its parent and allowing the node to be moved.
fn drag_start() -> impl Fn(
    Trigger<Pointer<DragStart>>,
    Commands,
    Query<(Entity, &mut Node, &mut Transform, &Parent), With<Dragable>>,
    ResMut<DragDetails>,
) {
    move |ev, mut commands, mut dragable, mut drag_details| {
        let Ok((item, mut item_node, mut transform, parent)) = dragable.get_mut(ev.target) else {
            return;
        };

        // Record which slot we started from, in case we need to drop the item back again.
        drag_details.drag_origin = Some(parent.get());

        // Absolutely-positioned items can be made to follow the cursor.
        item_node.position_type = PositionType::Absolute;

        // TODO: is there a better solution for the scaling problem? Without this, and without a
        // parent element, the asset will be un-scaled (and appear tiny). Copying its current
        // scaling doesn't seem to be helpful.
        transform.scale *= 4.;

        // Add a marker to tell our systems that the node is being dragged, and remove it from the
        // list of children on its current parent entity.
        commands.entity(item).insert(Dragging).remove_parent();
    }
}

/// This observer detects the end of our drag-and-drop, and checks the `DragDetails` resource to
/// decide where to place the node: either in a new `InventorySlot`, or back in the one it came
/// from.
fn drag_end() -> impl Fn(
    Trigger<Pointer<DragEnd>>,
    Commands,
    Query<(Entity, &mut Node, &mut Transform), With<Dragging>>,
    ResMut<DragDetails>,
) {
    move |_ev, mut commands, mut dragging_item, drag_details| {
        let Ok((item, mut item_node, mut transform)) = dragging_item.get_single_mut() else {
            return;
        };

        // Not dragging anymore!
        commands.entity(item).remove::<Dragging>();

        // We either want the new location, or the original one. In theory, both being `None`
        // shouldn't happen.
        let Some(drop_target) = drag_details.drop_target.or(drag_details.drag_origin) else {
            return;
        };

        // Whichever slot we pick, add the `Node` as a child entity.
        commands.entity(drop_target).add_child(item);

        // Re-integrate the `Node` into the relatively-positioned layout.
        item_node.position_type = PositionType::Relative;
        item_node.left = Val::Auto;
        item_node.top = Val::Auto;
        transform.scale /= 4.;
    }
}

/// This observer updates our record of which inventory slot is the current target. We
/// should also set the `drop_target` to `None` if the pointer is hovering over empty
/// space, for example. Even though all of our hovered `Node`s receive picking events, we don't
/// need to check the type of the `ev.target` because we've only attached this observer to
/// `InventorySlot`s.
fn set_drop_target() -> impl Fn(Trigger<Pointer<Over>>, ResMut<DragDetails>) {
    move |ev, mut details| {
        details.drop_target = Some(ev.target);
    }
}

/// This observer ensures that even if we'd previously hovered over an `InventorySlot`, we clear
/// the drop target again once the pointer leaves it.
fn clear_drop_target() -> impl Fn(Trigger<Pointer<Out>>, ResMut<DragDetails>) {
    move |_ev, mut details| {
        details.drop_target = None;
    }
}

/// Update the transform of the dragged item to follow the cursor.
fn drag_inventory_items(
    mut dragging: Query<&mut Node, With<Dragging>>,
    primary_window: Single<&Window, With<PrimaryWindow>>,
) {
    let Ok(mut item_node) = dragging.get_single_mut() else {
        return;
    };

    let Some(cursor_pos) = primary_window.cursor_position() else {
        return;
    };
    // We can't assign exact values with a `PositionType::Absolute` UI node. However, we can use
    // distance from top of viewport and distance from left of viewport to accomplish the same
    // thing.
    item_node.left = Val::Px(cursor_pos.x);
    item_node.top = Val::Px(cursor_pos.y);
}

/// Spawn two arbitrary inventory items. Here we'll just pick the first two slots.
fn add_items_to_inventory(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    slots: Query<Entity, With<InventorySlot>>,
) {
    let paths = [
        "textures/rpg/props/generic-rpg-loot01.png",
        "textures/rpg/props/generic-rpg-loot02.png",
    ];
    for (i, slot) in slots.iter().enumerate().take(2) {
        commands.entity(slot).with_children(|parent| {
            parent
                .spawn((
                    Dragable,
                    // This node serves solely to be dragged! It provides the transform, using required
                    // components.
                    Node::default(),
                    ImageNode::new(asset_server.load(paths[i])),
                ))
                .observe(drag_start())
                .observe(drag_end());
        });
    }
}

/// Display instructions.
fn spawn_text(mut commands: Commands) {
    commands.spawn((
        Name::new("Instructions"),
        Text::new("Drag and drop items within the inventory slots."),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.),
            left: Val::Px(12.),
            ..default()
        },
    ));
}