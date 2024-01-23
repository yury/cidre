// typedef NS_ENUM(NSInteger, NSWindowOrderingMode) {
//     NSWindowAbove		 =  1,
//     NSWindowBelow		 = -1,
//     NSWindowOut			 =  0
// };

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum WindowOrderingMode {
    Above = 1,
    Below = -1,
    Out = 0,
}
