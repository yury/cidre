use crate::{arc, define_cls, define_obj_type, ns, objc};

#[cfg(feature = "blocks")]
use crate::blocks;

define_obj_type!(
    #[doc(alias = "NSTimer")]
    pub Timer(ns::Id)
);

impl arc::A<Timer> {
    #[cfg(feature = "blocks")]
    #[objc::msg_send(initWithFireDate:interval:repeats:block:)]
    pub fn init_with_fire_date_block(
        self,
        date: &ns::Date,
        interval: ns::TimeInterval,
        repeats: bool,
        block: &mut blocks::SyncBlock<fn(timer: &mut ns::Timer)>,
    ) -> arc::R<Timer>;

    #[objc::msg_send(initWithFireDate:interval:target:selector:userInfo:repeats:)]
    pub fn init_with_fire_date_sel(
        self,
        date: &ns::Date,
        interval: ns::TimeInterval,
        target: &ns::Id,
        selector: &ns::Sel,
        user_info: Option<&ns::Id>,
        repeats: bool,
    ) -> arc::R<Timer>;
}

impl Timer {
    define_cls!(NS_TIMER);

    #[cfg(feature = "blocks")]
    #[objc::msg_send(scheduledTimerWithTimeInterval:repeats:block:)]
    pub fn scheduled_with_block(
        interval: ns::TimeInterval,
        repeats: bool,
        block: &mut blocks::SyncBlock<fn(timer: &mut ns::Timer)>,
    ) -> arc::R<Self>;

    #[cfg(feature = "blocks")]
    pub fn scheduled(
        interval: ns::TimeInterval,
        repeats: bool,
        block: impl FnMut(&mut ns::Timer) + 'static + Sync,
    ) -> arc::R<Self> {
        let mut block = blocks::SyncBlock::new1(block);
        Self::scheduled_with_block(interval, repeats, &mut block)
    }

    #[objc::msg_send(scheduledTimerWithTimeInterval:target:selector:userInfo:repeats:)]
    pub fn scheduled_with_sel(
        interval: ns::TimeInterval,
        target: &ns::Id,
        selector: &ns::Sel,
        user_info: Option<&ns::Id>,
        repeats: bool,
    ) -> arc::R<Self>;

    #[cfg(feature = "blocks")]
    pub fn with_fire_date_block(
        date: &ns::Date,
        interval: ns::TimeInterval,
        repeats: bool,
        block: &mut blocks::SyncBlock<fn(timer: &mut ns::Timer)>,
    ) -> arc::R<Self> {
        Self::alloc().init_with_fire_date_block(date, interval, repeats, block)
    }

    #[cfg(feature = "blocks")]
    pub fn with_fire_date(
        date: &ns::Date,
        interval: ns::TimeInterval,
        repeats: bool,
        block: impl FnMut(&mut ns::Timer) + 'static + Sync,
    ) -> arc::R<Self> {
        let mut block = blocks::SyncBlock::new1(block);
        Self::with_fire_date_block(date, interval, repeats, &mut block)
    }

    pub fn with_fire_date_sel(
        date: &ns::Date,
        interval: ns::TimeInterval,
        target: &ns::Id,
        selector: &ns::Sel,
        user_info: Option<&ns::Id>,
        repeats: bool,
    ) -> arc::R<Self> {
        Self::alloc().init_with_fire_date_sel(date, interval, target, selector, user_info, repeats)
    }

    #[objc::msg_send(timeInterval)]
    pub fn time_interval(&self) -> ns::TimeInterval;

    #[objc::msg_send(invalidate)]
    pub fn invalidate(&mut self);

    #[objc::msg_send(isValid)]
    pub fn is_valid(&self) -> bool;

    #[objc::msg_send(fire)]
    pub fn fire(&self);

    #[objc::msg_send(fireDate)]
    pub fn fire_date(&self) -> arc::R<ns::Date>;

    #[objc::msg_send(tolerance)]
    pub fn tolerance(&self) -> ns::TimeInterval;

    #[objc::msg_send(setTolerance:)]
    pub fn set_tolerance(&mut self, val: ns::TimeInterval);

    #[objc::msg_send(userInfo)]
    pub fn user_info(&self) -> Option<arc::R<ns::Id>>;

    #[objc::msg_send(setUserInfo:)]
    pub fn set_user_info(&mut self, val: Option<&ns::Id>);
}

#[cfg(feature = "cf")]
use crate::cf;

impl Timer {
    pub fn as_cf(&self) -> &cf::RunLoopTimer {
        unsafe { std::mem::transmute(self) }
    }
}

#[link(name = "ns", kind = "static")]
unsafe extern "C" {
    static NS_TIMER: &'static objc::Class<Timer>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let mut timer = ns::Timer::with_fire_date(&ns::Date::new(), 0.5, false, |_timer| {});
        assert_eq!(timer.user_info().is_none(), true);
        assert_eq!(timer.is_valid(), true);
        timer.invalidate();
        assert_eq!(timer.is_valid(), false);

        let cf_timer = timer.as_cf();
        eprintln!("{cf_timer:?}");
    }
}
