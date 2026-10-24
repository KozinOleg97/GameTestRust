use bevy::picking::hover::HoverMap;
use bevy::picking::pointer::PointerId;
use bevy::prelude::*;

/// Свойство панели интерфейса: перехватывает она указатель или нет.
///
/// Непрозрачная панель забирает клики и перетаскивания себе: мир под ней не
/// получает ни наведения, ни выбора. Прозрачная пропускает указатель сквозь
/// себя — так задаются рамки, подложки и отладочные панели, которые не должны
/// мешать смотреть на мир.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PointerOcclusion {
    #[default]
    Opaque,
    Transparent,
}

/// Курсор сейчас над непрозрачной панелью интерфейса.
///
/// Правило одно на всех потребителей: и выбор гекса, и перетаскивание камеры
/// спрашивают этот ресурс, а не разбираются с интерфейсом по-своему.
#[derive(Resource, Debug, Default)]
pub struct PointerBlocked(pub bool);

impl PointerBlocked {
    #[inline]
    pub fn get(&self) -> bool {
        self.0
    }
}

pub struct PointerPlugin;

impl Plugin for PointerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PointerBlocked>()
            .add_systems(Update, update_pointer_blocked);
    }
}

/// Пересчитывает перехват по карте попаданий пикинга.
///
/// Состояние считается заново каждый кадр, а не накапливается в ресурсе: тогда
/// удалённая или скрытая панель не может оставить указатель «залипшим».
pub fn update_pointer_blocked(
    hover_map: Option<Res<HoverMap>>,
    occlusions: Query<&PointerOcclusion>,
    parents: Query<&ChildOf>,
    mut blocked: ResMut<PointerBlocked>,
) {
    let Some(hover_map) = hover_map else {
        blocked.0 = false;
        return;
    };

    let Some(hits) = hover_map.0.get(&PointerId::Mouse) else {
        blocked.0 = false;
        return;
    };

    blocked.0 = hits
        .keys()
        .any(|entity| occludes_pointer(*entity, &occlusions, &parents));
}

/// Ищет непрозрачную панель на самой сущности или среди её предков.
///
/// Проверка предков важна: попадание почти всегда приходит в дочерний узел —
/// текст, кнопку, полосу прогресса, — а свойство задано на корне панели.
fn occludes_pointer(
    entity: Entity,
    occlusions: &Query<&PointerOcclusion>,
    parents: &Query<&ChildOf>,
) -> bool {
    let mut current = entity;

    loop {
        if let Ok(occlusion) = occlusions.get(current)
            && *occlusion == PointerOcclusion::Opaque
        {
            return true;
        }

        let Ok(child_of) = parents.get(current) else {
            return false;
        };

        current = child_of.parent();
    }
}
