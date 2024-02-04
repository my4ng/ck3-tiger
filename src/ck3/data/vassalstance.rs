use crate::block::Block;
use crate::context::ScopeContext;
use crate::db::{Db, DbKind};
use crate::everything::Everything;
use crate::game::GameFlags;
use crate::item::{Item, ItemLoader};
use crate::scopes::Scopes;
use crate::token::Token;
use crate::tooltipped::Tooltipped;
use crate::trigger::validate_trigger;
use crate::validator::Validator;

#[derive(Clone, Debug)]
pub struct VassalStance {}

inventory::submit! {
    ItemLoader::Normal(GameFlags::Ck3, Item::VassalStance, VassalStance::add)
}

impl VassalStance {
    pub fn add(db: &mut Db, key: Token, block: Block) {
        db.add(Item::VassalStance, key, block, Box::new(Self {}));
    }
}

impl DbKind for VassalStance {
    fn validate(&self, key: &Token, block: &Block, data: &Everything) {
        let mut vd = Validator::new(block, data);
        let mut sc = ScopeContext::new(Scopes::Character, key);
        sc.define_name("liege", Scopes::Character, key);

        vd.multi_field_validated_block("is_valid", |block, data| {
            validate_trigger(block, data, &mut sc, Tooltipped::No);
        });

        vd.field_script_value("score", &mut sc);
        vd.field_script_value("heir_score", &mut sc);
    }
}
