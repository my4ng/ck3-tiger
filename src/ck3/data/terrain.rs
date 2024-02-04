use crate::block::Block;
use crate::context::ScopeContext;
use crate::db::{Db, DbKind};
use crate::everything::Everything;
use crate::game::GameFlags;
use crate::item::{Item, ItemLoader};
use crate::modif::{validate_modifs, ModifKinds};
use crate::scopes::Scopes;
use crate::token::Token;
use crate::validate::validate_color;
use crate::validator::Validator;

#[derive(Clone, Debug)]
pub struct Terrain {}

inventory::submit! {
    ItemLoader::Normal(GameFlags::Ck3, Item::Terrain, Terrain::add)
}

impl Terrain {
    pub fn add(db: &mut Db, key: Token, block: Block) {
        db.add(Item::Terrain, key, block, Box::new(Self {}));
    }
}

impl DbKind for Terrain {
    fn validate(&self, key: &Token, block: &Block, data: &Everything) {
        let mut vd = Validator::new(block, data);
        let mut sc = ScopeContext::new(Scopes::None, key);

        vd.req_field("color");

        vd.field_numeric("movement_speed");
        vd.field_validated_block("color", validate_color);
        vd.field_validated_block("travel_danger_color", validate_color);
        vd.field_script_value("travel_danger_score", &mut sc);

        vd.field_validated_block("attacker_modifier", |block, data| {
            let vd = Validator::new(block, data);
            validate_modifs(block, data, ModifKinds::Terrain, vd);
        });
        vd.field_validated_block("defender_modifier", |block, data| {
            let vd = Validator::new(block, data);
            validate_modifs(block, data, ModifKinds::Terrain, vd);
        });
        vd.field_block("attacker_combat_effects"); // TODO
        vd.field_block("defender_combat_effects"); // TODO

        vd.field_numeric("combat_width");
        vd.field_bool("is_desert");
        vd.field_bool("is_jungle");
        vd.field_numeric("audio_parameter"); // TODO: ??

        vd.field_validated_block("province_modifier", |block, data| {
            let vd = Validator::new(block, data);
            validate_modifs(block, data, ModifKinds::Province, vd);
        });
    }
}
