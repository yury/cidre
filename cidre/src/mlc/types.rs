use crate::{arc, ns};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum DataType {
    Invalid = 0,

    /// The 32-bit floating-point data type.
    F32 = 1,

    /// The 16-bit floating-point data type.
    F16 = 3,

    /// Boolean data type.
    Bool = 4,

    /// The 64-bit integer data type
    I64 = 5,

    /// The 32-bit integer data type
    I32 = 7,

    /// The 8-bit integer data type
    I8 = 8,

    /// The 8-bit unsigned integer data type.
    U8 = 9,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum RandomInitializerType {
    Invalid = 0,

    /// The uniform random initializer type.
    Uniform = 1,

    /// The glorot uniform random initializer type.
    GlorotUniform = 2,

    /// The Xavier random initializer type.
    Xavier = 3,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum DeviceType {
    /// The CPU device
    CPU = 0,

    /// The GPU device
    GPU = 1,

    /// The any device type.  When selected, the framework will automatically use the appropriate devices
    /// to achieve the best performance.
    Any = 2,

    /// The  Apple Neural Engine device.  When selected, the framework will use the  Neural Engine to execute all layers that can be executed on it.
    /// Layers that cannot be executed on the ANE will run on the CPU or GPU.   The Neural Engine device must be explicitly selected.  MLDeviceTypeAny
    /// will not select the Neural Engine device.  In addition, this device can be used with inference graphs only.  This device cannot be used with a
    /// training graph or an inference graph that shares layers with a training graph.
    ///
    ANE = 3,
}

#[doc(alias = "MLCArithmeticOperation")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum ArithmeticOp {
    /// An operation that calculates the elementwise sum of its two inputs.
    Add = 0,

    /// An operation that calculates the elementwise difference of its two inputs.
    Subtract = 1,

    /// An operation that calculates the elementwise product of its two inputs.
    Multiply = 2,

    /// An operation that calculates the elementwise division of its two inputs.
    Divide = 3,

    /// An operation that calculates the elementwise floor of its two inputs.
    Floor = 4,

    /// An operation that calculates the elementwise round of its inputs.
    Round = 5,

    /// An operation that calculates the elementwise ceiling of its inputs.
    Ceil = 6,

    /// An operation that calculates the elementwise square root of its inputs.
    Sqrt = 7,

    /// An operation that calculates the elementwise reciprocal of the square root of its inputs.
    RSqrt = 8,

    /// An operation that calculates the elementwise sine of its inputs.
    Sin = 9,

    /// An operation that calculates the elementwise cosine of its inputs.
    Cos = 10,

    /// An operation that calculates the elementwise tangent of its inputs.
    Tan = 11,

    /// An operation that calculates the elementwise inverse sine of its inputs.
    ASin = 12,

    /// An operation that calculates the elementwise inverse cosine of its inputs.
    ACos = 13,

    /// An operation that calculates the elementwise inverse tangent of its inputs.
    ATan = 14,

    /// An operation that calculates the elementwise hyperbolic sine of its inputs.
    SinH = 15,

    /// An operation that calculates the elementwise hyperbolic cosine of its inputs.
    CosH = 16,

    /// An operation that calculates the elementwise hyperbolic tangent of its inputs.
    TanH = 17,

    /// An operation that calculates the elementwise inverse hyperbolic sine of its inputs.
    ASinH = 18,

    /// An operation that calculates the elementwise inverse hyperbolic cosine of its inputs.
    ACosH = 19,

    /// An operation that calculates the elementwise inverse hyperbolic tangent of its inputs.
    ATanH = 20,

    /// An operation that calculates the elementwise first input raised to the power of its second input.
    Pow = 21,

    /// An operation that calculates the elementwise result of e raised to the power of its input.
    Exp = 22,

    /// An operation that calculates the elementwise result of 2 raised to the power of its input.
    Exp2 = 23,

    /// An operation that calculates the elementwise natural logarithm of its input.
    Log = 24,

    /// An operation that calculates the elementwise base 2 logarithm of its input.
    Log2 = 25,

    /// An operation that calculates the elementwise product of its two inputs.  Returns 0 if y in x * y is zero, even if x is NaN or INF
    MultiplyNoNaN = 26,

    /// An operations that calculates the elementwise division of its two inputs.  Returns 0 if the denominator is 0.
    DivideNoNaN = 27,

    /// An operation that calculates the elementwise min of two inputs.
    Min = 28,

    /// An operations that calculates the elementwise max of two inputs.
    Max = 29,
}

impl ArithmeticOp {
    #[inline]
    pub fn debug_description(self) -> &'static ns::String {
        unsafe { MLCArithmeticOperationDebugDescription(self) }
    }
}

/// A loss function.
#[doc(alias = "MLCLossType")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum LossType {
    /// The mean absolute error loss.
    MeanAbsoluteError = 0,

    /// The mean squared error loss.
    MeanSquaredError = 1,

    /// The softmax cross entropy loss.
    SoftmaxCrossEntropy = 2,

    /// The sigmoid cross entropy loss.
    SigmoidCrossEntropy = 3,

    /// The categorical cross entropy loss.
    CategoricalCrossEntropy = 4,

    /// The hinge loss.
    Hinge = 5,

    /// The Huber loss.
    Huber = 6,

    /// The cosine distance loss.
    CosineDistance = 7,

    /// The log loss.
    Log = 8,
}

impl LossType {
    #[inline]
    pub fn debug_description(self) -> &'static ns::String {
        unsafe { MLCLossTypeDebugDescription(self) }
    }
}

/// An activation type that you specify for an activation descriptor.
#[doc(alias = "MLCActivationType")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum ActivationType {
    None = 0,

    /// The ReLU activation type.
    ///
    /// This activation type implements the following function:
    /// `f(x) = x >= 0 ? x : a * x`
    ReLU = 1,

    /// The linear activation type.
    ///
    /// This activation type implements the following function:
    /// `f(x) = a * x + b`
    Linear = 2,

    /// The sigmoid activation type.
    /// This activation type implements the following function:
    /// `f(x) = 1 / (1 + e⁻ˣ)`
    Sigmoid = 3,

    /// The hard sigmoid activation type.
    ///
    /// This activation type implements the following function:
    /// `f(x) = clamp((x * a) + b, 0, 1)`
    HardSigmoid = 4,

    /// The hyperbolic tangent (TanH) activation type.
    /// This activation type implements the following function:
    /// `f(x) = a * tanh(b * x)`
    TanH = 5,

    /// The absolute activation type.
    ///
    /// This activation type implements the following function:
    /// `f(x) = fabs(x)`
    Absolute = 6,

    /// The parametric soft plus activation type.
    ///
    /// This activation type implements the following function:
    /// `f(x) = a * log(1 + e^(b * x))`
    SoftPlus = 7,

    /// The parametric soft sign activation type.
    ///
    /// This activation type implements the following function:
    /// `f(x) = x / (1 + abs(x))`
    SoftSign = 8,

    /// The parametric ELU activation type.
    /// This activation type implements the following function:
    /// `f(x) = x >= 0 ? x : a * (exp(x) - 1)`
    ELU = 9,

    /// The ReLUN activation type.
    ///
    /// This activation type implements the following function:
    /// `f(x) = min((x >= 0 ? x : a * x), b)`
    ReLUN = 10,

    /// The log sigmoid activation type.
    ///
    /// This activation type implements the following function:
    ///  `f(x) = log(1 / (1 + exp(-x)))`
    LogSigmoid = 11,

    /// The SELU activation type.
    ///
    /// This activation type implements the following function:
    /// ```
    /// f(x) = scale * (max(0, x) + min(0, α * (exp(x) − 1)))
    /// ```
    /// where:
    /// ```
    /// α = 1.6732632423543772848170429916717
    /// scale = 1.0507009873554804934193349852946
    /// ```
    SELU = 12,

    /// The CELU activation type.
    ///
    /// This activation type implements the following function:
    /// ```
    /// f(x) = max(0, x) + min(0, a * (exp(x / a) − 1))
    /// ```
    CELU = 13,

    /// The hard shrink activation type.
    ///
    /// This activation type implements the following function:
    /// ```
    /// f(x) = x, if x > a or x < −a, else 0
    /// ```
    HardShrink = 14,

    /// The soft shrink activation type.
    ///
    /// This activation type implements the following function:
    /// ```
    /// f(x) = x - a, if x > a, x + a, if x < −a, else 0
    /// ```
    SoftShrink = 15,

    /// The hyperbolic tangent (TanH) shrink activation type.
    /// This activation type implements the following function:
    /// ```
    /// f(x) = x - tanh(x)
    /// ```
    TanHShrink = 16,

    /// The threshold activation type.
    ///
    /// This activation type implements the following function:
    /// ```
    ///  f(x) = x, if x > a, else b
    /// ```
    Threshold = 17,

    /// The GELU activation type.
    /// This activation type implements the following function:
    /// ```
    /// f(x) = x * CDF(x)
    /// ```
    GELU = 18,

    /// The hardswish activation type.
    ///
    /// This activation type implements the following function:
    /// ```
    /// f(x) = 0, if x <= -3
    /// f(x) = x, if x >= +3
    /// f(x) = x * (x + 3)/6, otherwise
    /// ```
    HardSwish = 19,

    /// The clamp activation type.
    /// This activation type implements the following function:
    /// ```
    ///  f(x) = min(max(x, a), b)
    /// ```
    Clamp = 20,
}

impl ActivationType {
    /// Returns a textual description of the arithmetic operation, suitable for debugging
    pub fn debug_description(self) -> &'static ns::String {
        unsafe { MLCActivationTypeDebugDescription(self) }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum ConvolutionType {
    /// The standard convolution type.
    Standard = 0,

    /// The transposed convolution type.
    Transposed = 1,

    /// The depthwise convolution type.
    Depthwise = 2,
}

impl ConvolutionType {
    pub fn debug_description(self) -> &'static ns::String {
        unsafe { MLCConvolutionTypeDebugDescription(self) }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum PaddingPolicy {
    /// The "same" padding policy.
    Same = 0,
    /// The "valid" padding policy.
    Valid = 1,
    /// The choice to use explicitly specified padding sizes.
    UsePaddingSize = 2,
}

impl PaddingPolicy {
    #[inline]
    pub fn debug_description(self) -> &'static ns::String {
        unsafe { MLCPaddingPolicyDebugDescription(self) }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum PaddingType {
    /// The zero padding type.
    Zero = 0,
    /// The reflect padding type.
    Reflect = 1,
    /// The symmetric padding type.
    Symmetric = 2,
    /// The constant padding type.
    Constant = 3,
}

impl PaddingType {
    pub fn debug_description(self) -> &'static ns::String {
        unsafe { MLCPaddingTypeDebugDescription(self) }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum PoolingType {
    /// The max pooling type.
    Max = 1,
    /// The average pooling type.
    Average = 2,
    /// The L2-norm pooling type.
    L2Norm = 3,
}

impl PoolingType {
    pub fn debug_description(self) -> &'static ns::String {
        unsafe { MLCPoolingTypeDebugDescription(self) }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum ReductionType {
    /// No reduction.
    None = 0,
    /// The sum reduction.
    Sum = 1,
    /// The mean reduction.
    Mean = 2,
    /// The max reduction.
    Max = 3,
    /// The min reduction.
    Min = 4,
    /// The argmax reduction.
    ArgMax = 5,
    /// The argmin reduction.
    ArgMin = 6,
    /// The L1norm reduction.
    L1Norm = 7,
    /// Any(X) = X_0 || X_1 || ... X_n
    Any = 8,
    /// Alf(X) = X_0 && X_1 && ... X_n
    All = 9,
}

impl ReductionType {
    #[inline]
    pub fn debug_description(self) -> &'static ns::String {
        unsafe { MLCReductionTypeDebugDescription(self) }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum RegularizationType {
    /// No regularization.
    None = 0,

    /// The L1 regularization.
    L1 = 1,

    /// The L2 regularization.
    L2 = 2,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum SampleMode {
    /// The nearest sample mode.
    Nearest = 0,
    /// The linear sample mode.
    Linear = 1,
}

impl SampleMode {
    pub fn debug_description(self) -> &'static ns::String {
        unsafe { MLCSampleModeDebugDescription(self) }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum SoftmaxOp {
    /// The standard softmax operation.
    Softmax = 0,
    /// The log softmax operation.
    LogSoftmax = 1,
}

impl SoftmaxOp {
    #[inline]
    pub fn debug_description(self) -> &'static ns::String {
        unsafe { MLCSoftmaxOperationDebugDescription(self) }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum LSTMResultMode {
    /// The output result mode. When selected for an LSTM layer, the layer will produce a single result tensor representing the final output of the LSTM.
    Output = 0,
    /// The output and states result mode. When selected for an LSTM layer, the layer will produce three result tensors representing the final output of
    ///  the LSTM, the last hidden state, and the cell state, respectively.
    OutputAndStates = 1,
}

impl LSTMResultMode {
    pub fn debug_description(self) -> &'static ns::String {
        unsafe { MLCLSTMResultModeDebugDescription(self) }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum ComparisonOp {
    Equal = 0,
    NotEqual = 1,
    Less = 2,
    Greater = 3,
    LessOrEqual = 4,
    GreaterOrEqual = 5,
    LogicalAND = 6,
    LogicalOR = 7,
    LogicalNOT = 8,
    LogicalNAND = 9,
    LogicalNOR = 10,
    LogicalXOR = 11,
}

impl ComparisonOp {
    pub fn debug_description(self) -> &'static ns::String {
        unsafe { MLCComparisonOperationDebugDescription(self) }
    }
}

/// The type of clipping applied to gradient
#[doc(alias = "MLCGradientClippingType")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum GradientClippingType {
    ByValue = 0,
    ByNorm = 1,
    ByGlobalNorm = 2,
}

impl GradientClippingType {
    pub fn debug_description(self) -> &'static ns::String {
        unsafe { MLCGradientClippingTypeDebugDescription(self) }
    }
}

#[link(name = "MLCompute", kind = "framework")]
extern "C" {
    fn MLCActivationTypeDebugDescription(activationType: ActivationType) -> &'static ns::String;
    fn MLCArithmeticOperationDebugDescription(op: ArithmeticOp) -> &'static ns::String;
    fn MLCPaddingPolicyDebugDescription(policy: PaddingPolicy) -> &'static ns::String;
    fn MLCLossTypeDebugDescription(loss_type: LossType) -> &'static ns::String;
    fn MLCReductionTypeDebugDescription(reduction_type: ReductionType) -> &'static ns::String;
    fn MLCPaddingTypeDebugDescription(padding_type: PaddingType) -> &'static ns::String;
    fn MLCConvolutionTypeDebugDescription(convolution_type: ConvolutionType)
        -> &'static ns::String;
    fn MLCPoolingTypeDebugDescription(pooling_type: PoolingType) -> &'static ns::String;
    fn MLCSoftmaxOperationDebugDescription(operation: SoftmaxOp) -> &'static ns::String;
    fn MLCSampleModeDebugDescription(mode: SampleMode) -> &'static ns::String;
    fn MLCLSTMResultModeDebugDescription(mode: LSTMResultMode) -> &'static ns::String;
    fn MLCComparisonOperationDebugDescription(operation: ComparisonOp) -> &'static ns::String;
    fn MLCGradientClippingTypeDebugDescription(
        gradient_clipping_type: GradientClippingType,
    ) -> &'static ns::String;
}

#[cfg(test)]
mod tests {
    use crate::mlc;

    #[test]
    fn basics() {
        let desc = mlc::ActivationType::ReLU.debug_description();
        assert_eq!(desc.to_string(), "ReLU")
    }
}
