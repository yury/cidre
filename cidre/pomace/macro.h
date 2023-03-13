//
//  macro.h
//  pomace
//
//  Created by Yury Korolev on 17.04.2022.
//

#ifndef macro_h
#define macro_h

NS_ASSUME_NONNULL_BEGIN

#define csel1(Prefix, ClassType, SEL_A, A, RetType) \
extern inline RetType Prefix ## ClassType ## _ ## SEL_A(A a) { return  [ClassType SEL_A: a]; } \

NS_ASSUME_NONNULL_END

#endif /* macro_h */
