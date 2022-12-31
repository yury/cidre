//
//  macro.h
//  pomace
//
//  Created by Yury Korolev on 17.04.2022.
//

#ifndef macro_h
#define macro_h

NS_ASSUME_NONNULL_BEGIN

#define rsel0(Prefix, SelfType, SEL, ReadType) \
ReadType Prefix ## rsel ## _ ## SEL(SelfType _self) { return  [_self SEL]; } \
\

#define rsel1(Prefix, SelfType, SEL_A, A, ReadType) \
extern inline ReadType Prefix ## rsel ## _ ## SEL_A(SelfType _self, A a) { return  [_self SEL_A: a]; } \
\

#define rsel2(Prefix, SelfType, SEL_A, A, SEL_B, B, ReadType) \
extern inline ReadType Prefix ## rsel ## _ ## SEL_A ## _ ## SEL_B(SelfType _self, A a, B b) { return  [_self SEL_A: a SEL_B: b]; } \
\

#define rsel3(Prefix, SelfType, SEL_A, A, SEL_B, B, SEL_C, C, ReadType) \
extern inline ReadType Prefix ## rsel ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C(SelfType _self, A a, B b, C c) { \
  return  [_self SEL_A: a SEL_B: b SEL_C: c]; } \
\

#define rsel4(Prefix, SelfType, SEL_A, A, SEL_B, B, SEL_C, C, SEL_D, D, ReadType) \
extern inline ReadType Prefix ## rsel ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C ## _ ## SEL_D(SelfType _self, A a, B b, C c, D d) { \
  return  [_self SEL_A: a SEL_B: b SEL_C: c SEL_D: d]; } \
\

#define rsel5(Prefix, SelfType, SEL_A, A, SEL_B, B, SEL_C, C, SEL_D, D, SEL_E, E, ReadType) \
extern inline ReadType Prefix ## rsel ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C ## _ ## SEL_D ## _ ## SEL_E(SelfType _self, A a, B b, C c, D d, E e) { \
  return  [_self SEL_A: a SEL_B: b SEL_C: c SEL_D: d SEL_E: e]; } \
\

#define rsel6(Prefix, SelfType, SEL_A, A, SEL_B, B, SEL_C, C, SEL_D, D, SEL_E, E, SEL_F, F, ReadType) \
extern inline ReadType Prefix ## rsel ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C ## _ ## SEL_D ## _ ## SEL_E ## _ ## SEL_F(SelfType _self, A a, B b, C c, D d, E e, F f) { \
  return  [_self SEL_A: a SEL_B: b SEL_C: c SEL_D: d SEL_E: e SEL_F: f]; } \
\



#define wsel0(Prefix, SelfType, SEL) \
extern inline void Prefix ## wsel ## _ ## SEL(SelfType _self) { [_self SEL]; } \
\

#define wsel1(Prefix, SelfType, SEL_A, A) \
extern inline void Prefix ## wsel ## _ ## SEL_A(SelfType _self, A a) { [_self SEL_A: a]; } \
\

#define wsel2(Prefix, SelfType, SEL_A, A, SEL_B, B) \
extern inline void Prefix ## wsel ## _ ## SEL_A ## _ ## SEL_B(SelfType _self, A a, B b) { [_self SEL_A: a SEL_B: b]; } \
\

#define wsel3(Prefix, SelfType, SEL_A, A, SEL_B, B, SEL_C, C) \
extern inline void Prefix ## wsel ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C(SelfType _self, A a, B b, C c) { [_self SEL_A: a SEL_B: b SEL_C: c]; } \
\

#define wsel4(Prefix, SelfType, SEL_A, A, SEL_B, B, SEL_C, C, SEL_D, D) \
extern inline void Prefix ## wsel ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C_ ## SEL_D(SelfType _self, A a, B b, C c, D d) { [_self SEL_A: a SEL_B: b SEL_C: c SEL_D: d]; } \
\

#define wsel5(Prefix, SelfType, SEL_A, A, SEL_B, B, SEL_C, C, SEL_D, D, SEL_E, E) \
extern inline void Prefix ## wsel ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C ## _ ## SEL_D ## _ ## SEL_E(SelfType _self, A a, B b, C c, D d, E e) { [_self SEL_A: a SEL_B: b SEL_C: c SEL_D: d SEL_E: e]; } \
\

#define wsel6(Prefix, SelfType, SEL_A, A, SEL_B, B, SEL_C, C, SEL_D, D, SEL_E, E) \
extern inline void Prefix ## wsel ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C ## _ ## SEL_D ## _ ## SEL_E ## _ ## SEL_F(SelfType _self, A a, B b, C c, D d, E e, F f) { [_self SEL_A: a SEL_B: b SEL_C: c SEL_D: d SEL_E: e SEL_F: f]; } \
\

#define wsel7(Prefix, SelfType, SEL_A, A, SEL_B, B, SEL_C, C, SEL_D, D, SEL_E, E) \
extern inline void Prefix ## wsel ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C ## _ ## SEL_D ## _ ## SEL_E ## _ ## SEL_F ## _ ## SEL_G(SelfType _self, A a, B b, C c, D d, E e, F f, G g) { [_self SEL_A: a SEL_B: b SEL_C: c SEL_D: d SEL_E: e SEL_F: f SEL_G: g]; } \
\

#define wsel8(Prefix, SelfType, SEL_A, A, SEL_B, B, SEL_C, C, SEL_D, D, SEL_E, E) \
extern inline void Prefix ## wsel ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C ## _ ## SEL_D ## _ ## SEL_E ## _ ## SEL_F ## _ ## SEL_G ## _ ## SEL_H(SelfType _self, A a, B b, C c, D d, E e, F f, G g, H h) { [_self SEL_A: a SEL_B: b SEL_C: c SEL_D: d SEL_E: e SEL_F: f SEL_G: g SEL_H: h]; } \
\

#define wsel9(Prefix, SelfType, SEL_A, A, SEL_B, B, SEL_C, C, SEL_D, D, SEL_E, E, SEL_F, F, SEL_G, G, SEL_H, H, SEL_I, I) \
extern inline void Prefix ## wsel ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C ## _ ## SEL_D ## _ ## SEL_E ## _ ## SEL_F ## _ ## SEL_G ## _ ## SEL_H ## _ ## SEL_I(SelfType _self, A a, B b, C c, D d, E e, F f, G g, H h, I i) { [_self SEL_A: a SEL_B: b SEL_C: c SEL_D: d SEL_E: e SEL_F: f SEL_G: g SEL_H: h SEL_I: i]; } \
\




#define rwsel(Prefix, SelfType, ReadSel, WriteSel, Type) \
rsel0(Prefix, SelfType, ReadSel, Type) \
wsel1(Prefix, SelfType, WriteSel, Type) \

#define asel0(Prefix, ClassType, SEL) \
extern inline ClassType * Prefix ## ClassType ## _ ## SEL(void) { return  [[ClassType alloc] SEL]; } \

#define asel1(Prefix, ClassType, SEL_A, A) \
extern inline ClassType * Prefix ## ClassType ## _ ## SEL_A(A a) { return  [[ClassType alloc] SEL_A: a]; } \

#define asel2(Prefix, ClassType, SEL_A, A, SEL_B, B) \
extern inline ClassType * Prefix ## ClassType ## _ ## SEL_A ## _ ## SEL_B(A a, B b) { return  [[ClassType alloc] SEL_A: a SEL_B: b]; } \

#define asel3(Prefix, ClassType, SEL_A, A, SEL_B, B, SEL_C, C) \
extern inline ClassType * Prefix ## ClassType ## _ ## SEL_A ## _ ## SEL_B_ ## SEL_C(A a, B b, C c) { return  [[ClassType alloc] SEL_A: a SEL_B: b SEL_C: c]; } \

#define asel4(Prefix, ClassType, SEL_A, A, SEL_B, B, SEL_C, C, SEL_D, D) \
extern inline ClassType * Prefix ## ClassType ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C ## _ ## SEL_D(A a, B b, C c, D d) { return  [[ClassType alloc] SEL_A: a SEL_B: b SEL_C: c SEL_D: d]; } \


#define csel0(Prefix, ClassType, SEL, RetType) \
extern inline RetType Prefix ## ClassType ## _ ## SEL(void) { return  [ClassType SEL]; } \

#define csel1(Prefix, ClassType, SEL_A, A, RetType) \
extern inline RetType Prefix ## ClassType ## _ ## SEL_A(A a) { return  [ClassType SEL_A: a]; } \

#define csel2(Prefix, ClassType, SEL_A, A, SEL_B, B, RetType) \
extern inline RetType Prefix ## ClassType ## _ ## SEL_A ## _ ## SEL_B(A a, B b) { return  [ClassType SEL_A: a SEL_B: b]; } \

#define csel3(Prefix, ClassType, SEL_A, A, SEL_B, B, SEL_C, C, RetType) \
extern inline RetType Prefix ## ClassType ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C(A a, B b, C c) { return  [ClassType SEL_A: a SEL_B: b SEL_C: c]; } \

#define csel4(Prefix, ClassType, SEL_A, A, SEL_B, B, SEL_C, C, SEL_D, D, RetType) \
extern inline RetType Prefix ## ClassType ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C ## _ ## SEL_D(A a, B b, C c, D d) { return  [ClassType SEL_A: a SEL_B: b SEL_C: c SEL_D: d]; } \

#define csel5(Prefix, ClassType, SEL_A, A, SEL_B, B, SEL_C, C, SEL_D, D, SEL_E, E, RetType) \
extern inline RetType Prefix ## ClassType ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C ## _ ## SEL_D ## _ ## SEL_E(A a, B b, C c, D d, E e) { return  [ClassType SEL_A: a SEL_B: b SEL_C: c SEL_D: d SEL_E: e]; } \

#define csel6(Prefix, ClassType, SEL_A, A, SEL_B, B, SEL_C, C, SEL_D, D, SEL_E, E, SEL_F, F, RetType) \
extern inline RetType Prefix ## ClassType ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C ## _ ## SEL_D ## _ ## SEL_E ## _ ## SEL_F(A a, B b, C c, D d, E e, F f) { return  [ClassType SEL_A: a SEL_B: b SEL_C: c SEL_D: d SEL_E: e SEL_F: f]; } \

#define csel7(Prefix, ClassType, SEL_A, A, SEL_B, B, SEL_C, C, SEL_D, D, SEL_E, E, SEL_F, F, SEL_G, G, RetType) \
extern inline RetType Prefix ## ClassType ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C ## _ ## SEL_D ## _ ## SEL_E ## _ ## SEL_F ## _ ## SEL_G(A a, B b, C c, D d, E e, F f, G g) { return  [ClassType SEL_A: a SEL_B: b SEL_C: c SEL_D: d SEL_E: e SEL_F: f SEL_G: g]; } \

#define csel8(Prefix, ClassType, SEL_A, A, SEL_B, B, SEL_C, C, SEL_D, D, SEL_E, E, SEL_F, F, SEL_G, G, SEL_H, H, RetType) \
extern inline RetType Prefix ## ClassType ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C ## _ ## SEL_D ## _ ## SEL_E ## _ ## SEL_F ## _ ## SEL_G ## _ ## SEL_H(A a, B b, C c, D d, E e, F f, G g, H h) { return  [ClassType SEL_A: a SEL_B: b SEL_C: c SEL_D: d SEL_E: e SEL_F: f SEL_G: g SEL_H: h]; } \

#define csel9(Prefix, ClassType, SEL_A, A, SEL_B, B, SEL_C, C, SEL_D, D, SEL_E, E, SEL_F, F, SEL_G, G, SEL_H, H, SEL_I, I, RetType) \
extern inline RetType Prefix ## ClassType ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C ## _ ## SEL_D ## _ ## SEL_E ## _ ## SEL_F ## _ ## SEL_G ## _ ## SEL_H ## _ ## SEL_I(A a, B b, C c, D d, E e, F f, G g, H h, I i) { return  [ClassType SEL_A: a SEL_B: b SEL_C: c SEL_D: d SEL_E: e SEL_F: f SEL_G: g SEL_H: h SEL_I: i]; } \

#define csel10(Prefix, ClassType, SEL_A, A, SEL_B, B, SEL_C, C, SEL_D, D, SEL_E, E, SEL_F, F, SEL_G, G, SEL_H, H, SEL_I, I, SEL_J, J, RetType) \
extern inline RetType Prefix ## ClassType ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C ## _ ## SEL_D ## _ ## SEL_E ## _ ## SEL_F ## _ ## SEL_G ## _ ## SEL_H ## _ ## SEL_I ## _ ## SEL_J(A a, B b, C c, D d, E e, F f, G g, H h, I i, J j) { return  [ClassType SEL_A: a SEL_B: b SEL_C: c SEL_D: d SEL_E: e SEL_F: f SEL_G: g SEL_H: h SEL_I: i SEL_J: j]; } \

#define csel11(Prefix, ClassType, SEL_A, A, SEL_B, B, SEL_C, C, SEL_D, D, SEL_E, E, SEL_F, F, SEL_G, G, SEL_H, H, SEL_I, I, SEL_J, J, SEL_K, K, RetType) \
extern inline RetType Prefix ## ClassType ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C ## _ ## SEL_D ## _ ## SEL_E ## _ ## SEL_F ## _ ## SEL_G ## _ ## SEL_H ## _ ## SEL_I ## _ ## SEL_J ## _ ## SEL_K(A a, B b, C c, D d, E e, F f, G g, H h, I i, J j, K k) { return  [ClassType SEL_A: a SEL_B: b SEL_C: c SEL_D: d SEL_E: e SEL_F: f SEL_G: g SEL_H: h SEL_I: i SEL_J: j SEL_K: k]; } \

#define csel12(Prefix, ClassType, SEL_A, A, SEL_B, B, SEL_C, C, SEL_D, D, SEL_E, E, SEL_F, F, SEL_G, G, SEL_H, H, SEL_I, I, SEL_J, J, SEL_K, K, SEL_L, L, RetType) \
extern inline RetType Prefix ## ClassType ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C ## _ ## SEL_D ## _ ## SEL_E ## _ ## SEL_F ## _ ## SEL_G ## _ ## SEL_H ## _ ## SEL_I ## _ ## SEL_J ## _ ## SEL_K ## _ ## SEL_L(A a, B b, C c, D d, E e, F f, G g, H h, I i, J j, K k, L l) { return  [ClassType SEL_A: a SEL_B: b SEL_C: c SEL_D: d SEL_E: e SEL_F: f SEL_G: g SEL_H: h SEL_I: i SEL_J: j SEL_K: k SEL_L: l]; } \

#define csel13(Prefix, ClassType, SEL_A, A, SEL_B, B, SEL_C, C, SEL_D, D, SEL_E, E, SEL_F, F, SEL_G, G, SEL_H, H, SEL_I, I, SEL_J, J, SEL_K, K, SEL_L, L, SEL_M, M, RetType) \
extern inline RetType Prefix ## ClassType ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C ## _ ## SEL_D ## _ ## SEL_E ## _ ## SEL_F ## _ ## SEL_G ## _ ## SEL_H ## _ ## SEL_I ## _ ## SEL_J ## _ ## SEL_K ## _ ## SEL_L ## _ ## SEL_M(A a, B b, C c, D d, E e, F f, G g, H h, I i, J j, K k, L l, M m) { return  [ClassType SEL_A: a SEL_B: b SEL_C: c SEL_D: d SEL_E: e SEL_F: f SEL_G: g SEL_H: h SEL_I: i SEL_J: j SEL_K: k SEL_L: l SEL_M: m]; } \

#define cinit2(Prefix, ClassType, SEL_A, A, SEL_B, B) \
extern inline ClassType * Prefix ## ClassType ## _ ## SEL_A ## _ ## SEL_B(A a, B b) { return  [[ClassType alloc] SEL_A: a SEL_B: b]; } \

#define cinit3(Prefix, ClassType, SEL_A, A, SEL_B, B, SEL_C, C) \
extern inline ClassType * Prefix ## ClassType ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C(A a, B b, C c) { return  [[ClassType alloc] SEL_A: a SEL_B: b SEL_C: c]; } \


NS_ASSUME_NONNULL_END

#endif /* macro_h */
