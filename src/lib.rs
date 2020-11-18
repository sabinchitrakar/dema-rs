#![feature(external_doc)]
use ta_common::traits::Indicator;
use ema_rs::EMA;
use ta_common::fixed_queue::FixedQueue;
#[doc(include = "../README.md")]
pub struct DEMA {
    period: u32,
    ema: EMA,
    ema2: EMA,
    ema_current: FixedQueue<f64>,
    ema2_current: FixedQueue<f64>,

}

impl DEMA {
    pub fn new(period: u32) -> DEMA {
        Self {
            period,
            ema: EMA::new(period),
            ema2: EMA::new(period),
            ema_current: FixedQueue::new(period),
            ema2_current: FixedQueue::new(period),
        }
    }
}

impl Indicator<f64, Option<f64>> for DEMA {
    fn next(&mut self, input: f64) -> Option<f64> {
        let ema = self.ema.next(input);
        self.ema_current.add(ema);
        let last_index = (self.period - 1) as i32;
        if self.ema_current.is_full() {
            let ema2 = self.ema2.next(self.ema_current.at(last_index).unwrap());
            self.ema2_current.add(ema2);
        }
        return if self.ema_current.is_full() && self.ema2_current.is_full() {
            let res = self.ema_current.at(last_index)
                .and_then(|ema| self.ema2_current.at(last_index)
                .map(|ema2| 2_f64 * ema - ema2));
            res
        } else {
            None
        }
    }

    fn reset(&mut self) {
        self.ema.reset();
        self.ema2.reset();
    }
}

#[cfg(test)]
mod tests {
    use crate::DEMA;
    use ta_common::traits::Indicator;

    #[test]
    fn it_works() {
        let mut ema = DEMA::new(5);
        assert_eq!(ema.next(81.59), None);
        assert_eq!(ema.next(81.06), None);
        assert_eq!(ema.next(82.87), None);
        assert_eq!(ema.next(83.00), None);
        assert_eq!(ema.next(83.61), None);
        assert_eq!(ema.next(83.15), None);
        assert_eq!(ema.next(82.84), None);
        assert_eq!(ema.next(83.99), None);
        assert_eq!(ema.next(84.55), Some(84.15945181120765));
        assert_eq!(ema.next(84.36), Some(84.37935138613696));
        assert_eq!(ema.next(85.53), Some(85.12493437653475));
        assert_eq!(ema.next(86.54), Some(86.06242299709658));
        assert_eq!(ema.next(86.89), Some(86.72659316211335));
        assert_eq!(ema.next(87.77), Some(87.5288251063304));
        assert_eq!(ema.next(87.29), Some(87.64550318083462));
    }
}
