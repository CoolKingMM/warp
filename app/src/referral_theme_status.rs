use warpui::{Entity, ModelContext};

#[derive(Default)]
pub struct ReferralThemeStatus;

#[derive(Clone, Debug)]
pub enum ReferralThemeEvent {}

impl Entity for ReferralThemeStatus {
    type Event = ReferralThemeEvent;
}

impl ReferralThemeStatus {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self
    }

    pub fn query_remote_status(&mut self, _ctx: &mut ModelContext<Self>) {}

    pub fn sent_referral_theme_active(&self) -> bool {
        false
    }

    pub fn received_referral_theme_active(&self) -> bool {
        false
    }
}
