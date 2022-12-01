//
//  macro.h
//  pomace
//
//  Created by Yury Korolev on 17.04.2022.
//

#ifndef macro_h
#define macro_h

NS_ASSUME_NONNULL_BEGIN


#define wsel(Prefix, SelfType, SEL) \
extern inline void Prefix ## wsel ## _ ## SEL(SelfType _self) { [_self SEL]; } \
\

#define rsel(Prefix, SelfType, SEL, ReadType) \
ReadType Prefix ## rsel ## _ ## SEL(SelfType _self) { return  [_self SEL]; } \
\

#define rsel_a(Prefix, SelfType, SEL_A, A, ReadType) \
extern inline ReadType Prefix ## rsel ## _ ## SEL_A(SelfType _self, A a) { return  [_self SEL_A: a]; } \
\

#define rsel_ab(Prefix, SelfType, SEL_A, A, SEL_B, B, ReadType) \
extern inline ReadType Prefix ## rsel ## _ ## SEL_A ## _ ## SEL_B(SelfType _self, A a, B b) { return  [_self SEL_A: a SEL_B: b]; } \
\

#define rsel_abc(Prefix, SelfType, SEL_A, A, SEL_B, B, SEL_C, C, ReadType) \
extern inline ReadType Prefix ## rsel ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C(SelfType _self, A a, B b, C c) { \
  return  [_self SEL_A: a SEL_B: b SEL_C: c]; } \
\

#define rsel_abcd(Prefix, SelfType, SEL_A, A, SEL_B, B, SEL_C, C, SEL_D, D, ReadType) \
extern inline ReadType Prefix ## rsel ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C ## _ ## SEL_D(SelfType _self, A a, B b, C c, D d) { \
  return  [_self SEL_A: a SEL_B: b SEL_C: c SEL_D: d]; } \
\

#define rsel_abcde(Prefix, SelfType, SEL_A, A, SEL_B, B, SEL_C, C, SEL_D, D, SEL_E, E, ReadType) \
extern inline ReadType Prefix ## rsel ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C ## _ ## SEL_D ## _ ## SEL_E(SelfType _self, A a, B b, C c, D d, E e) { \
  return  [_self SEL_A: a SEL_B: b SEL_C: c SEL_D: d SEL_E: e]; } \
\

#define rsel_abcdef(Prefix, SelfType, SEL_A, A, SEL_B, B, SEL_C, C, SEL_D, D, SEL_E, E, SEL_F, F, ReadType) \
extern inline ReadType Prefix ## rsel ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C ## _ ## SEL_D ## _ ## SEL_E ## _ ## SEL_F(SelfType _self, A a, B b, C c, D d, E e, F f) { \
  return  [_self SEL_A: a SEL_B: b SEL_C: c SEL_D: d SEL_E: e SEL_F: f]; } \
\

#define wsel_a(Prefix, SelfType, SEL_A, A) \
extern inline void Prefix ## wsel ## _ ## SEL_A(SelfType _self, A a) { [_self SEL_A: a]; } \
\

#define wsel_ab(Prefix, SelfType, SEL_A, A, SEL_B, B) \
extern inline void Prefix ## wsel ## _ ## SEL_A ## _ ## SEL_B(SelfType _self, A a, B b) { [_self SEL_A: a SEL_B: b]; } \
\

#define wsel_abc(Prefix, SelfType, SEL_A, A, SEL_B, B, SEL_C, C) \
extern inline void Prefix ## wsel ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C(SelfType _self, A a, B b, C c) { [_self SEL_A: a SEL_B: b SEL_C: c]; } \
\

#define wsel_abcd(Prefix, SelfType, SEL_A, A, SEL_B, B, SEL_C, C, SEL_D, D) \
extern inline void Prefix ## wsel ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C_ ## SEL_D(SelfType _self, A a, B b, C c, D d) { [_self SEL_A: a SEL_B: b SEL_C: c SEL_D: d]; } \
\

#define wsel_abcde(Prefix, SelfType, SEL_A, A, SEL_B, B, SEL_C, C, SEL_D, D, SEL_E, E) \
extern inline void Prefix ## wsel ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C ## _ ## SEL_D ## _ ## SEL_E(SelfType _self, A a, B b, C c, D d, E e) { [_self SEL_A: a SEL_B: b SEL_C: c SEL_D: d SEL_E: e]; } \
\

#define wsel_abcdef(Prefix, SelfType, SEL_A, A, SEL_B, B, SEL_C, C, SEL_D, D, SEL_E, E) \
extern inline void Prefix ## wsel ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C ## _ ## SEL_D ## _ ## SEL_E ## _ ## SEL_F(SelfType _self, A a, B b, C c, D d, E e, F f) { [_self SEL_A: a SEL_B: b SEL_C: c SEL_D: d SEL_E: e SEL_F: f]; } \
\

#define wsel_abcdefg(Prefix, SelfType, SEL_A, A, SEL_B, B, SEL_C, C, SEL_D, D, SEL_E, E) \
extern inline void Prefix ## wsel ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C ## _ ## SEL_D ## _ ## SEL_E ## _ ## SEL_F ## _ ## SEL_G(SelfType _self, A a, B b, C c, D d, E e, F f, G g) { [_self SEL_A: a SEL_B: b SEL_C: c SEL_D: d SEL_E: e SEL_F: f SEL_G: g]; } \
\

#define wsel_abcdefgh(Prefix, SelfType, SEL_A, A, SEL_B, B, SEL_C, C, SEL_D, D, SEL_E, E) \
extern inline void Prefix ## wsel ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C ## _ ## SEL_D ## _ ## SEL_E ## _ ## SEL_F ## _ ## SEL_G ## _ ## SEL_H(SelfType _self, A a, B b, C c, D d, E e, F f, G g, H h) { [_self SEL_A: a SEL_B: b SEL_C: c SEL_D: d SEL_E: e SEL_F: f SEL_G: g SEL_H: h]; } \
\

#define wsel_abcdefghi(Prefix, SelfType, SEL_A, A, SEL_B, B, SEL_C, C, SEL_D, D, SEL_E, E, SEL_F, F, SEL_G, G, SEL_H, H, SEL_I, I) \
extern inline void Prefix ## wsel ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C ## _ ## SEL_D ## _ ## SEL_E ## _ ## SEL_F ## _ ## SEL_G ## _ ## SEL_H ## _ ## SEL_I(SelfType _self, A a, B b, C c, D d, E e, F f, G g, H h, I i) { [_self SEL_A: a SEL_B: b SEL_C: c SEL_D: d SEL_E: e SEL_F: f SEL_G: g SEL_H: h SEL_I: i]; } \
\




#define rwsel(Prefix, SelfType, ReadSel, WriteSel, Type) \
rsel(Prefix, SelfType, ReadSel, Type) \
wsel_a(Prefix, SelfType, WriteSel, Type) \

#define asel(Prefix, ClassType, SEL) \
extern inline ClassType * Prefix ## ClassType ## _ ## SEL(void) { return  [[ClassType alloc] SEL]; } \

#define asel_a(Prefix, ClassType, SEL_A, A) \
extern inline ClassType * Prefix ## ClassType ## _ ## SEL_A(A a) { return  [[ClassType alloc] SEL_A: a]; } \

#define asel_ab(Prefix, ClassType, SEL_A, A, SEL_B, B) \
extern inline ClassType * Prefix ## ClassType ## _ ## SEL_A ## _ ## SEL_B(A a, B b) { return  [[ClassType alloc] SEL_A: a SEL_B: b]; } \

#define asel_abc(Prefix, ClassType, SEL_A, A, SEL_B, B, SEL_C, C) \
extern inline ClassType * Prefix ## ClassType ## _ ## SEL_A ## _ ## SEL_B_ ## SEL_C(A a, B b, C c) { return  [[ClassType alloc] SEL_A: a SEL_B: b SEL_C: c]; } \

#define asel_abcd(Prefix, ClassType, SEL_A, A, SEL_B, B, SEL_C, C, SEL_D, D) \
extern inline ClassType * Prefix ## ClassType ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C ## _ ## SEL_D(A a, B b, C c, D d) { return  [[ClassType alloc] SEL_A: a SEL_B: b SEL_C: c SEL_D: d]; } \


#define csel(Prefix, ClassType, SEL, RetType) \
extern inline RetType Prefix ## ClassType ## _ ## SEL(void) { return  [ClassType SEL]; } \

#define csel_a(Prefix, ClassType, SEL_A, A, RetType) \
extern inline RetType Prefix ## ClassType ## _ ## SEL_A(A a) { return  [ClassType SEL_A: a]; } \

#define csel_ab(Prefix, ClassType, SEL_A, A, SEL_B, B, RetType) \
extern inline RetType Prefix ## ClassType ## _ ## SEL_A ## _ ## SEL_B(A a, B b) { return  [ClassType SEL_A: a SEL_B: b]; } \

#define csel_abc(Prefix, ClassType, SEL_A, A, SEL_B, B, SEL_C, C, RetType) \
extern inline RetType Prefix ## ClassType ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C(A a, B b, C c) { return  [ClassType SEL_A: a SEL_B: b SEL_C: c]; } \

#define csel_abcd(Prefix, ClassType, SEL_A, A, SEL_B, B, SEL_C, C, SEL_D, D, RetType) \
extern inline RetType Prefix ## ClassType ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C ## _ ## SEL_D(A a, B b, C c, D d) { return  [ClassType SEL_A: a SEL_B: b SEL_C: c SEL_D: d]; } \

#define cinit_ab(Prefix, ClassType, SEL_A, A, SEL_B, B) \
extern inline ClassType * Prefix ## ClassType ## _ ## SEL_A ## _ ## SEL_B(A a, B b) { return  [[ClassType alloc] SEL_A: a SEL_B: b]; } \

#define cinit_abc(Prefix, ClassType, SEL_A, A, SEL_B, B, SEL_C, C) \
extern inline ClassType * Prefix ## ClassType ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C(A a, B b, C c) { return  [[ClassType alloc] SEL_A: a SEL_B: b SEL_C: c]; } \


NS_ASSUME_NONNULL_END

#endif /* macro_h */
