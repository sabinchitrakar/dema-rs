# Double Exponential Moving Average
```
use dema_rs::DEMA;
use ta_common::traits::Indicator;
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
```

### Calculation  
DEMA=2*EMA<sub>n</sub>(input) -EMA(EMA<sub>n</sub>(input));  
n=period