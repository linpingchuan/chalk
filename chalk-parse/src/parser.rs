use ast::*;
use lalrpop_intern::{intern, InternedString};
use std::iter::once;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Program {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use ast::*;
    use lalrpop_intern::{intern, InternedString};
    use std::iter::once;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2c_22(&'input str),
        Term_22_2d_3e_22(&'input str),
        Term_22_2e_22(&'input str),
        Term_22_3a_2d_22(&'input str),
        Term_22_3b_22(&'input str),
        Term_22_3d_3e_22(&'input str),
        Term_22___22(&'input str),
        Term_22exists_22(&'input str),
        Term_22forall_22(&'input str),
        Termr_23_22_5c_27_5b_5e_5c_27_5d_2b_5c_27_22_23(&'input str),
        Termr_23_22_5b_2d_7c_21_40_23_24_25_5e_26_2a_3d_2b_2f_3a_3f_7e_3c_3e_5d_2b_22_23(&'input str),
        Termr_23_22_5bA_2dZa_2dz_5d_5bA_2dZa_2dz0_2d9___5d_2a_22_23(&'input str),
        Termr_23_22_5bA_2dZa_2dz_5d_5bA_2dZa_2dz0_2d9___5d_2a_3a_22_23(&'input str),
        Nt_28BitOperator_20BitValue_29((Bit, Bit)),
        Nt_28BitOperator_20BitValue_29_2b(::std::vec::Vec<(Bit, Bit)>),
        Nt_40L(usize),
        Nt_40R(usize),
        NtApplication(Application),
        NtApplicationBits(Vec<Bit>),
        NtAtom(Atom),
        NtBitApplication(Bit),
        NtBitApplications(Vec<Bit>),
        NtBitOperator(Bit),
        NtBitOperator_3f(::std::option::Option<Bit>),
        NtBitValue(Bit),
        NtBitValue_3f(::std::option::Option<Bit>),
        NtFact_3cFactData_3e(Fact),
        NtFact_3cFactDataAnd_3e(Fact),
        NtFact_3cFactDataApply_3e(Fact),
        NtFact_3cFactDataFunc_3e(Fact),
        NtFact_3cFactDataOr_3e(Fact),
        NtFactData(Box<FactData>),
        NtFactDataAnd(Box<FactData>),
        NtFactDataApply(Box<FactData>),
        NtFactDataFunc(Box<FactData>),
        NtFactDataOr(Box<FactData>),
        NtIdentifier(InternedString),
        NtItem(Item),
        NtItem_2b(::std::vec::Vec<Item>),
        NtOperator(Operator),
        NtOperatorValue((Operator, Value)),
        NtProgram(Program),
        NtRule(Rule),
        NtValue(Value),
        NtVariable(Variable),
        NtVec1_3cBitApplication_3e(Vec<Bit>),
        Nt____Program(Program),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        16, // on "(", goto 15
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        17, // on "_", goto 16
        0, // on "exists", error
        0, // on "forall", error
        18, // on r#"\'[^\']+\'"#, goto 17
        19, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 18
        20, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 19
        21, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 20
        // State 1
        0, // on "(", error
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        -12, // on ".", reduce `ApplicationBits = (BitOperator BitValue)+ => ActionFn(82);`
        -12, // on ":-", reduce `ApplicationBits = (BitOperator BitValue)+ => ActionFn(82);`
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        19, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 18
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        21, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 20
        // State 2
        0, // on "(", error
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        23, // on ".", goto 22
        24, // on ":-", goto 23
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 3
        0, // on "(", error
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        -6, // on ".", reduce `Application = ApplicationBits => ActionFn(66);`
        -6, // on ":-", reduce `Application = ApplicationBits => ActionFn(66);`
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 4
        0, // on "(", error
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        -48, // on ".", reduce `Value = Atom => ActionFn(27);`
        -48, // on ":-", reduce `Value = Atom => ActionFn(27);`
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -48, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Value = Atom => ActionFn(27);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -48, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Value = Atom => ActionFn(27);`
        // State 5
        16, // on "(", goto 15
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        -8, // on ".", reduce `ApplicationBits = BitOperator => ActionFn(17);`
        -8, // on ":-", reduce `ApplicationBits = BitOperator => ActionFn(17);`
        0, // on ";", error
        0, // on "=>", error
        17, // on "_", goto 16
        0, // on "exists", error
        0, // on "forall", error
        18, // on r#"\'[^\']+\'"#, goto 17
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        27, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 26
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 6
        0, // on "(", error
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        -7, // on ".", reduce `ApplicationBits = BitValue => ActionFn(16);`
        -7, // on ":-", reduce `ApplicationBits = BitValue => ActionFn(16);`
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        31, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 30
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        32, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 31
        // State 7
        33, // on "(", goto 32
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        -52, // on ".", reduce `Variable = Identifier => ActionFn(32);`
        -52, // on ":-", reduce `Variable = Identifier => ActionFn(32);`
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -52, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Variable = Identifier => ActionFn(32);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -52, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Variable = Identifier => ActionFn(32);`
        // State 8
        -41, // on "(", reduce `Item+ = Item => ActionFn(51);`
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        -41, // on "_", reduce `Item+ = Item => ActionFn(51);`
        0, // on "exists", error
        0, // on "forall", error
        -41, // on r#"\'[^\']+\'"#, reduce `Item+ = Item => ActionFn(51);`
        -41, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Item+ = Item => ActionFn(51);`
        -41, // on r#"[A-Za-z][A-Za-z0-9_]*"#, reduce `Item+ = Item => ActionFn(51);`
        -41, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Item+ = Item => ActionFn(51);`
        // State 9
        16, // on "(", goto 15
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        17, // on "_", goto 16
        0, // on "exists", error
        0, // on "forall", error
        18, // on r#"\'[^\']+\'"#, goto 17
        19, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 18
        20, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 19
        21, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 20
        // State 10
        -17, // on "(", reduce `BitOperator = Operator => ActionFn(69);`
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        -17, // on ".", reduce `BitOperator = Operator => ActionFn(69);`
        -17, // on ":-", reduce `BitOperator = Operator => ActionFn(69);`
        0, // on ";", error
        0, // on "=>", error
        -17, // on "_", reduce `BitOperator = Operator => ActionFn(69);`
        0, // on "exists", error
        0, // on "forall", error
        -17, // on r#"\'[^\']+\'"#, reduce `BitOperator = Operator => ActionFn(69);`
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        -17, // on r#"[A-Za-z][A-Za-z0-9_]*"#, reduce `BitOperator = Operator => ActionFn(69);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 11
        0, // on "(", error
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 12
        -40, // on "(", reduce `Item = Rule => ActionFn(3);`
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        -40, // on "_", reduce `Item = Rule => ActionFn(3);`
        0, // on "exists", error
        0, // on "forall", error
        -40, // on r#"\'[^\']+\'"#, reduce `Item = Rule => ActionFn(3);`
        -40, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Item = Rule => ActionFn(3);`
        -40, // on r#"[A-Za-z][A-Za-z0-9_]*"#, reduce `Item = Rule => ActionFn(3);`
        -40, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Item = Rule => ActionFn(3);`
        // State 13
        0, // on "(", error
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        -20, // on ".", reduce `BitValue = Value => ActionFn(70);`
        -20, // on ":-", reduce `BitValue = Value => ActionFn(70);`
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -20, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `BitValue = Value => ActionFn(70);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -20, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `BitValue = Value => ActionFn(70);`
        // State 14
        0, // on "(", error
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        -49, // on ".", reduce `Value = Variable => ActionFn(28);`
        -49, // on ":-", reduce `Value = Variable => ActionFn(28);`
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -49, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Value = Variable => ActionFn(28);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -49, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Value = Variable => ActionFn(28);`
        // State 15
        45, // on "(", goto 44
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        46, // on "_", goto 45
        0, // on "exists", error
        0, // on "forall", error
        47, // on r#"\'[^\']+\'"#, goto 46
        48, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 47
        49, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 48
        50, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 49
        // State 16
        0, // on "(", error
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        -51, // on ".", reduce `Value = "_" => ActionFn(30);`
        -51, // on ":-", reduce `Value = "_" => ActionFn(30);`
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -51, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Value = "_" => ActionFn(30);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -51, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Value = "_" => ActionFn(30);`
        // State 17
        0, // on "(", error
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        -14, // on ".", reduce `Atom = r#"\'[^\']+\'"# => ActionFn(31);`
        -14, // on ":-", reduce `Atom = r#"\'[^\']+\'"# => ActionFn(31);`
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -14, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Atom = r#"\'[^\']+\'"# => ActionFn(31);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -14, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Atom = r#"\'[^\']+\'"# => ActionFn(31);`
        // State 18
        -44, // on "(", reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        -44, // on ".", reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        -44, // on ":-", reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        0, // on ";", error
        0, // on "=>", error
        -44, // on "_", reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        0, // on "exists", error
        0, // on "forall", error
        -44, // on r#"\'[^\']+\'"#, reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        -44, // on r#"[A-Za-z][A-Za-z0-9_]*"#, reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 19
        -38, // on "(", reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        -38, // on ".", reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        -38, // on ":-", reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -38, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -38, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        // State 20
        -43, // on "(", reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        -43, // on ".", reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        -43, // on ":-", reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        0, // on ";", error
        0, // on "=>", error
        -43, // on "_", reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        0, // on "exists", error
        0, // on "forall", error
        -43, // on r#"\'[^\']+\'"#, reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        -43, // on r#"[A-Za-z][A-Za-z0-9_]*"#, reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 21
        16, // on "(", goto 15
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        -10, // on ".", reduce `ApplicationBits = (BitOperator BitValue)+, BitOperator => ActionFn(80);`
        -10, // on ":-", reduce `ApplicationBits = (BitOperator BitValue)+, BitOperator => ActionFn(80);`
        0, // on ";", error
        0, // on "=>", error
        17, // on "_", goto 16
        0, // on "exists", error
        0, // on "forall", error
        18, // on r#"\'[^\']+\'"#, goto 17
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        27, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 26
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 22
        -39, // on "(", reduce `Item = Application, "." => ActionFn(2);`
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        -39, // on "_", reduce `Item = Application, "." => ActionFn(2);`
        0, // on "exists", error
        0, // on "forall", error
        -39, // on r#"\'[^\']+\'"#, reduce `Item = Application, "." => ActionFn(2);`
        -39, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Item = Application, "." => ActionFn(2);`
        -39, // on r#"[A-Za-z][A-Za-z0-9_]*"#, reduce `Item = Application, "." => ActionFn(2);`
        -39, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Item = Application, "." => ActionFn(2);`
        // State 23
        71, // on "(", goto 70
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        72, // on "_", goto 71
        73, // on "exists", goto 72
        74, // on "forall", goto 73
        75, // on r#"\'[^\']+\'"#, goto 74
        76, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 75
        77, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 76
        78, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 77
        // State 24
        0, // on "(", error
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        -2, // on ".", reduce `(BitOperator BitValue)+ = BitOperator, BitValue => ActionFn(53);`
        -2, // on ":-", reduce `(BitOperator BitValue)+ = BitOperator, BitValue => ActionFn(53);`
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -2, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `(BitOperator BitValue)+ = BitOperator, BitValue => ActionFn(53);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -2, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `(BitOperator BitValue)+ = BitOperator, BitValue => ActionFn(53);`
        // State 25
        0, // on "(", error
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        -52, // on ".", reduce `Variable = Identifier => ActionFn(32);`
        -52, // on ":-", reduce `Variable = Identifier => ActionFn(32);`
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -52, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Variable = Identifier => ActionFn(32);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -52, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Variable = Identifier => ActionFn(32);`
        // State 26
        0, // on "(", error
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        -38, // on ".", reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        -38, // on ":-", reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -38, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -38, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        // State 27
        0, // on "(", error
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        -11, // on ".", reduce `ApplicationBits = BitValue, (BitOperator BitValue)+ => ActionFn(81);`
        -11, // on ":-", reduce `ApplicationBits = BitValue, (BitOperator BitValue)+ => ActionFn(81);`
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        19, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 18
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        21, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 20
        // State 28
        16, // on "(", goto 15
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        17, // on "_", goto 16
        0, // on "exists", error
        0, // on "forall", error
        18, // on r#"\'[^\']+\'"#, goto 17
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        27, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 26
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 29
        -17, // on "(", reduce `BitOperator = Operator => ActionFn(69);`
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        -17, // on "_", reduce `BitOperator = Operator => ActionFn(69);`
        0, // on "exists", error
        0, // on "forall", error
        -17, // on r#"\'[^\']+\'"#, reduce `BitOperator = Operator => ActionFn(69);`
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        -17, // on r#"[A-Za-z][A-Za-z0-9_]*"#, reduce `BitOperator = Operator => ActionFn(69);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 30
        -44, // on "(", reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        -44, // on "_", reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        0, // on "exists", error
        0, // on "forall", error
        -44, // on r#"\'[^\']+\'"#, reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        -44, // on r#"[A-Za-z][A-Za-z0-9_]*"#, reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 31
        -43, // on "(", reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        -43, // on "_", reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        0, // on "exists", error
        0, // on "forall", error
        -43, // on r#"\'[^\']+\'"#, reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        -43, // on r#"[A-Za-z][A-Za-z0-9_]*"#, reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 32
        93, // on "(", goto 92
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        94, // on "_", goto 93
        0, // on "exists", error
        0, // on "forall", error
        95, // on r#"\'[^\']+\'"#, goto 94
        96, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 95
        97, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 96
        98, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 97
        // State 33
        -42, // on "(", reduce `Item+ = Item+, Item => ActionFn(52);`
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        -42, // on "_", reduce `Item+ = Item+, Item => ActionFn(52);`
        0, // on "exists", error
        0, // on "forall", error
        -42, // on r#"\'[^\']+\'"#, reduce `Item+ = Item+, Item => ActionFn(52);`
        -42, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Item+ = Item+, Item => ActionFn(52);`
        -42, // on r#"[A-Za-z][A-Za-z0-9_]*"#, reduce `Item+ = Item+, Item => ActionFn(52);`
        -42, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Item+ = Item+, Item => ActionFn(52);`
        // State 34
        0, // on "(", error
        -12, // on ")", reduce `ApplicationBits = (BitOperator BitValue)+ => ActionFn(82);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        48, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 47
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        50, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 49
        // State 35
        0, // on "(", error
        100, // on ")", goto 99
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 36
        0, // on "(", error
        -6, // on ")", reduce `Application = ApplicationBits => ActionFn(66);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 37
        0, // on "(", error
        -48, // on ")", reduce `Value = Atom => ActionFn(27);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -48, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Value = Atom => ActionFn(27);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -48, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Value = Atom => ActionFn(27);`
        // State 38
        45, // on "(", goto 44
        -8, // on ")", reduce `ApplicationBits = BitOperator => ActionFn(17);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        46, // on "_", goto 45
        0, // on "exists", error
        0, // on "forall", error
        47, // on r#"\'[^\']+\'"#, goto 46
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        103, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 102
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 39
        0, // on "(", error
        -7, // on ")", reduce `ApplicationBits = BitValue => ActionFn(16);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        31, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 30
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        32, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 31
        // State 40
        106, // on "(", goto 105
        -52, // on ")", reduce `Variable = Identifier => ActionFn(32);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -52, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Variable = Identifier => ActionFn(32);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -52, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Variable = Identifier => ActionFn(32);`
        // State 41
        -17, // on "(", reduce `BitOperator = Operator => ActionFn(69);`
        -17, // on ")", reduce `BitOperator = Operator => ActionFn(69);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        -17, // on "_", reduce `BitOperator = Operator => ActionFn(69);`
        0, // on "exists", error
        0, // on "forall", error
        -17, // on r#"\'[^\']+\'"#, reduce `BitOperator = Operator => ActionFn(69);`
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        -17, // on r#"[A-Za-z][A-Za-z0-9_]*"#, reduce `BitOperator = Operator => ActionFn(69);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 42
        0, // on "(", error
        -20, // on ")", reduce `BitValue = Value => ActionFn(70);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -20, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `BitValue = Value => ActionFn(70);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -20, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `BitValue = Value => ActionFn(70);`
        // State 43
        0, // on "(", error
        -49, // on ")", reduce `Value = Variable => ActionFn(28);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -49, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Value = Variable => ActionFn(28);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -49, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Value = Variable => ActionFn(28);`
        // State 44
        45, // on "(", goto 44
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        46, // on "_", goto 45
        0, // on "exists", error
        0, // on "forall", error
        47, // on r#"\'[^\']+\'"#, goto 46
        48, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 47
        49, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 48
        50, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 49
        // State 45
        0, // on "(", error
        -51, // on ")", reduce `Value = "_" => ActionFn(30);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -51, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Value = "_" => ActionFn(30);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -51, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Value = "_" => ActionFn(30);`
        // State 46
        0, // on "(", error
        -14, // on ")", reduce `Atom = r#"\'[^\']+\'"# => ActionFn(31);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -14, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Atom = r#"\'[^\']+\'"# => ActionFn(31);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -14, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Atom = r#"\'[^\']+\'"# => ActionFn(31);`
        // State 47
        -44, // on "(", reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        -44, // on ")", reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        -44, // on "_", reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        0, // on "exists", error
        0, // on "forall", error
        -44, // on r#"\'[^\']+\'"#, reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        -44, // on r#"[A-Za-z][A-Za-z0-9_]*"#, reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 48
        -38, // on "(", reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        -38, // on ")", reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -38, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -38, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        // State 49
        -43, // on "(", reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        -43, // on ")", reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        -43, // on "_", reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        0, // on "exists", error
        0, // on "forall", error
        -43, // on r#"\'[^\']+\'"#, reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        -43, // on r#"[A-Za-z][A-Za-z0-9_]*"#, reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 50
        0, // on "(", error
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        -3, // on ".", reduce `(BitOperator BitValue)+ = (BitOperator BitValue)+, BitOperator, BitValue => ActionFn(54);`
        -3, // on ":-", reduce `(BitOperator BitValue)+ = (BitOperator BitValue)+, BitOperator, BitValue => ActionFn(54);`
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -3, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `(BitOperator BitValue)+ = (BitOperator BitValue)+, BitOperator, BitValue => ActionFn(54);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -3, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `(BitOperator BitValue)+ = (BitOperator BitValue)+, BitOperator, BitValue => ActionFn(54);`
        // State 51
        0, // on "(", error
        0, // on ")", error
        -12, // on ",", reduce `ApplicationBits = (BitOperator BitValue)+ => ActionFn(82);`
        0, // on "->", error
        -12, // on ".", reduce `ApplicationBits = (BitOperator BitValue)+ => ActionFn(82);`
        0, // on ":-", error
        -12, // on ";", reduce `ApplicationBits = (BitOperator BitValue)+ => ActionFn(82);`
        -12, // on "=>", reduce `ApplicationBits = (BitOperator BitValue)+ => ActionFn(82);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        76, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 75
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        78, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 77
        // State 52
        0, // on "(", error
        0, // on ")", error
        -31, // on ",", reduce `FactDataApply = Application => ActionFn(14);`
        0, // on "->", error
        -31, // on ".", reduce `FactDataApply = Application => ActionFn(14);`
        0, // on ":-", error
        -31, // on ";", reduce `FactDataApply = Application => ActionFn(14);`
        -31, // on "=>", reduce `FactDataApply = Application => ActionFn(14);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 53
        0, // on "(", error
        0, // on ")", error
        -6, // on ",", reduce `Application = ApplicationBits => ActionFn(66);`
        0, // on "->", error
        -6, // on ".", reduce `Application = ApplicationBits => ActionFn(66);`
        0, // on ":-", error
        -6, // on ";", reduce `Application = ApplicationBits => ActionFn(66);`
        -6, // on "=>", reduce `Application = ApplicationBits => ActionFn(66);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 54
        0, // on "(", error
        0, // on ")", error
        -48, // on ",", reduce `Value = Atom => ActionFn(27);`
        0, // on "->", error
        -48, // on ".", reduce `Value = Atom => ActionFn(27);`
        0, // on ":-", error
        -48, // on ";", reduce `Value = Atom => ActionFn(27);`
        -48, // on "=>", reduce `Value = Atom => ActionFn(27);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -48, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Value = Atom => ActionFn(27);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -48, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Value = Atom => ActionFn(27);`
        // State 55
        71, // on "(", goto 70
        0, // on ")", error
        -8, // on ",", reduce `ApplicationBits = BitOperator => ActionFn(17);`
        0, // on "->", error
        -8, // on ".", reduce `ApplicationBits = BitOperator => ActionFn(17);`
        0, // on ":-", error
        -8, // on ";", reduce `ApplicationBits = BitOperator => ActionFn(17);`
        -8, // on "=>", reduce `ApplicationBits = BitOperator => ActionFn(17);`
        72, // on "_", goto 71
        0, // on "exists", error
        0, // on "forall", error
        75, // on r#"\'[^\']+\'"#, goto 74
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        111, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 110
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 56
        0, // on "(", error
        0, // on ")", error
        -7, // on ",", reduce `ApplicationBits = BitValue => ActionFn(16);`
        0, // on "->", error
        -7, // on ".", reduce `ApplicationBits = BitValue => ActionFn(16);`
        0, // on ":-", error
        -7, // on ";", reduce `ApplicationBits = BitValue => ActionFn(16);`
        -7, // on "=>", reduce `ApplicationBits = BitValue => ActionFn(16);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        31, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 30
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        32, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 31
        // State 57
        0, // on "(", error
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        114, // on ".", goto 113
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 58
        0, // on "(", error
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        115, // on ";", goto 114
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 59
        0, // on "(", error
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        116, // on "=>", goto 115
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 60
        0, // on "(", error
        0, // on ")", error
        117, // on ",", goto 116
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 61
        0, // on "(", error
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        -23, // on ".", reduce `Fact<FactData> = FactData => ActionFn(71);`
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 62
        0, // on "(", error
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        -28, // on ".", reduce `FactData = FactDataAnd => ActionFn(5);`
        0, // on ":-", error
        -24, // on ";", reduce `Fact<FactDataAnd> = FactDataAnd => ActionFn(72);`
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 63
        0, // on "(", error
        0, // on ")", error
        -32, // on ",", reduce `FactDataFunc = FactDataApply => ActionFn(10);`
        0, // on "->", error
        -32, // on ".", reduce `FactDataFunc = FactDataApply => ActionFn(10);`
        0, // on ":-", error
        -32, // on ";", reduce `FactDataFunc = FactDataApply => ActionFn(10);`
        -25, // on "=>", reduce `Fact<FactDataApply> = FactDataApply => ActionFn(73);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 64
        0, // on "(", error
        0, // on ")", error
        -36, // on ",", reduce `FactDataOr = FactDataFunc => ActionFn(8);`
        0, // on "->", error
        -36, // on ".", reduce `FactDataOr = FactDataFunc => ActionFn(8);`
        0, // on ":-", error
        -36, // on ";", reduce `FactDataOr = FactDataFunc => ActionFn(8);`
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 65
        0, // on "(", error
        0, // on ")", error
        -27, // on ",", reduce `Fact<FactDataOr> = FactDataOr => ActionFn(75);`
        0, // on "->", error
        -29, // on ".", reduce `FactDataAnd = FactDataOr => ActionFn(6);`
        0, // on ":-", error
        -29, // on ";", reduce `FactDataAnd = FactDataOr => ActionFn(6);`
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 66
        118, // on "(", goto 117
        0, // on ")", error
        -52, // on ",", reduce `Variable = Identifier => ActionFn(32);`
        0, // on "->", error
        -52, // on ".", reduce `Variable = Identifier => ActionFn(32);`
        0, // on ":-", error
        -52, // on ";", reduce `Variable = Identifier => ActionFn(32);`
        -52, // on "=>", reduce `Variable = Identifier => ActionFn(32);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -52, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Variable = Identifier => ActionFn(32);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -52, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Variable = Identifier => ActionFn(32);`
        // State 67
        -17, // on "(", reduce `BitOperator = Operator => ActionFn(69);`
        0, // on ")", error
        -17, // on ",", reduce `BitOperator = Operator => ActionFn(69);`
        0, // on "->", error
        -17, // on ".", reduce `BitOperator = Operator => ActionFn(69);`
        0, // on ":-", error
        -17, // on ";", reduce `BitOperator = Operator => ActionFn(69);`
        -17, // on "=>", reduce `BitOperator = Operator => ActionFn(69);`
        -17, // on "_", reduce `BitOperator = Operator => ActionFn(69);`
        0, // on "exists", error
        0, // on "forall", error
        -17, // on r#"\'[^\']+\'"#, reduce `BitOperator = Operator => ActionFn(69);`
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        -17, // on r#"[A-Za-z][A-Za-z0-9_]*"#, reduce `BitOperator = Operator => ActionFn(69);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 68
        0, // on "(", error
        0, // on ")", error
        -20, // on ",", reduce `BitValue = Value => ActionFn(70);`
        0, // on "->", error
        -20, // on ".", reduce `BitValue = Value => ActionFn(70);`
        0, // on ":-", error
        -20, // on ";", reduce `BitValue = Value => ActionFn(70);`
        -20, // on "=>", reduce `BitValue = Value => ActionFn(70);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -20, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `BitValue = Value => ActionFn(70);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -20, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `BitValue = Value => ActionFn(70);`
        // State 69
        0, // on "(", error
        0, // on ")", error
        -49, // on ",", reduce `Value = Variable => ActionFn(28);`
        0, // on "->", error
        -49, // on ".", reduce `Value = Variable => ActionFn(28);`
        0, // on ":-", error
        -49, // on ";", reduce `Value = Variable => ActionFn(28);`
        -49, // on "=>", reduce `Value = Variable => ActionFn(28);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -49, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Value = Variable => ActionFn(28);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -49, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Value = Variable => ActionFn(28);`
        // State 70
        45, // on "(", goto 44
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        46, // on "_", goto 45
        0, // on "exists", error
        0, // on "forall", error
        47, // on r#"\'[^\']+\'"#, goto 46
        48, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 47
        49, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 48
        50, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 49
        // State 71
        0, // on "(", error
        0, // on ")", error
        -51, // on ",", reduce `Value = "_" => ActionFn(30);`
        0, // on "->", error
        -51, // on ".", reduce `Value = "_" => ActionFn(30);`
        0, // on ":-", error
        -51, // on ";", reduce `Value = "_" => ActionFn(30);`
        -51, // on "=>", reduce `Value = "_" => ActionFn(30);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -51, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Value = "_" => ActionFn(30);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -51, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Value = "_" => ActionFn(30);`
        // State 72
        120, // on "(", goto 119
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 73
        121, // on "(", goto 120
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 74
        0, // on "(", error
        0, // on ")", error
        -14, // on ",", reduce `Atom = r#"\'[^\']+\'"# => ActionFn(31);`
        0, // on "->", error
        -14, // on ".", reduce `Atom = r#"\'[^\']+\'"# => ActionFn(31);`
        0, // on ":-", error
        -14, // on ";", reduce `Atom = r#"\'[^\']+\'"# => ActionFn(31);`
        -14, // on "=>", reduce `Atom = r#"\'[^\']+\'"# => ActionFn(31);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -14, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Atom = r#"\'[^\']+\'"# => ActionFn(31);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -14, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Atom = r#"\'[^\']+\'"# => ActionFn(31);`
        // State 75
        -44, // on "(", reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        0, // on ")", error
        -44, // on ",", reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        0, // on "->", error
        -44, // on ".", reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        0, // on ":-", error
        -44, // on ";", reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        -44, // on "=>", reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        -44, // on "_", reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        0, // on "exists", error
        0, // on "forall", error
        -44, // on r#"\'[^\']+\'"#, reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        -44, // on r#"[A-Za-z][A-Za-z0-9_]*"#, reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 76
        -38, // on "(", reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        0, // on ")", error
        -38, // on ",", reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        0, // on "->", error
        -38, // on ".", reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        0, // on ":-", error
        -38, // on ";", reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        -38, // on "=>", reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -38, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -38, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        // State 77
        -43, // on "(", reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        0, // on ")", error
        -43, // on ",", reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        0, // on "->", error
        -43, // on ".", reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        0, // on ":-", error
        -43, // on ";", reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        -43, // on "=>", reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        -43, // on "_", reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        0, // on "exists", error
        0, // on "forall", error
        -43, // on r#"\'[^\']+\'"#, reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        -43, // on r#"[A-Za-z][A-Za-z0-9_]*"#, reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 78
        16, // on "(", goto 15
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        -9, // on ".", reduce `ApplicationBits = BitValue, (BitOperator BitValue)+, BitOperator => ActionFn(79);`
        -9, // on ":-", reduce `ApplicationBits = BitValue, (BitOperator BitValue)+, BitOperator => ActionFn(79);`
        0, // on ";", error
        0, // on "=>", error
        17, // on "_", goto 16
        0, // on "exists", error
        0, // on "forall", error
        18, // on r#"\'[^\']+\'"#, goto 17
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        27, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 26
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 79
        0, // on "(", error
        -12, // on ")", reduce `ApplicationBits = (BitOperator BitValue)+ => ActionFn(82);`
        -12, // on ",", reduce `ApplicationBits = (BitOperator BitValue)+ => ActionFn(82);`
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        96, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 95
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        98, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 97
        // State 80
        0, // on "(", error
        -15, // on ")", reduce `BitApplication = Application => ActionFn(68);`
        -15, // on ",", reduce `BitApplication = Application => ActionFn(68);`
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 81
        0, // on "(", error
        -6, // on ")", reduce `Application = ApplicationBits => ActionFn(66);`
        -6, // on ",", reduce `Application = ApplicationBits => ActionFn(66);`
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 82
        0, // on "(", error
        -48, // on ")", reduce `Value = Atom => ActionFn(27);`
        -48, // on ",", reduce `Value = Atom => ActionFn(27);`
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -48, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Value = Atom => ActionFn(27);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -48, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Value = Atom => ActionFn(27);`
        // State 83
        0, // on "(", error
        -53, // on ")", reduce `Vec1<BitApplication> = BitApplication => ActionFn(34);`
        -53, // on ",", reduce `Vec1<BitApplication> = BitApplication => ActionFn(34);`
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 84
        0, // on "(", error
        123, // on ")", goto 122
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 85
        93, // on "(", goto 92
        -8, // on ")", reduce `ApplicationBits = BitOperator => ActionFn(17);`
        -8, // on ",", reduce `ApplicationBits = BitOperator => ActionFn(17);`
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        94, // on "_", goto 93
        0, // on "exists", error
        0, // on "forall", error
        95, // on r#"\'[^\']+\'"#, goto 94
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        126, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 125
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 86
        0, // on "(", error
        -7, // on ")", reduce `ApplicationBits = BitValue => ActionFn(16);`
        -7, // on ",", reduce `ApplicationBits = BitValue => ActionFn(16);`
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        31, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 30
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        32, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 31
        // State 87
        129, // on "(", goto 128
        -52, // on ")", reduce `Variable = Identifier => ActionFn(32);`
        -52, // on ",", reduce `Variable = Identifier => ActionFn(32);`
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -52, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Variable = Identifier => ActionFn(32);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -52, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Variable = Identifier => ActionFn(32);`
        // State 88
        -17, // on "(", reduce `BitOperator = Operator => ActionFn(69);`
        -17, // on ")", reduce `BitOperator = Operator => ActionFn(69);`
        -17, // on ",", reduce `BitOperator = Operator => ActionFn(69);`
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        -17, // on "_", reduce `BitOperator = Operator => ActionFn(69);`
        0, // on "exists", error
        0, // on "forall", error
        -17, // on r#"\'[^\']+\'"#, reduce `BitOperator = Operator => ActionFn(69);`
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        -17, // on r#"[A-Za-z][A-Za-z0-9_]*"#, reduce `BitOperator = Operator => ActionFn(69);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 89
        0, // on "(", error
        -20, // on ")", reduce `BitValue = Value => ActionFn(70);`
        -20, // on ",", reduce `BitValue = Value => ActionFn(70);`
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -20, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `BitValue = Value => ActionFn(70);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -20, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `BitValue = Value => ActionFn(70);`
        // State 90
        0, // on "(", error
        -49, // on ")", reduce `Value = Variable => ActionFn(28);`
        -49, // on ",", reduce `Value = Variable => ActionFn(28);`
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -49, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Value = Variable => ActionFn(28);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -49, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Value = Variable => ActionFn(28);`
        // State 91
        0, // on "(", error
        -16, // on ")", reduce `BitApplications = Vec1<BitApplication> => ActionFn(20);`
        130, // on ",", goto 129
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 92
        45, // on "(", goto 44
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        46, // on "_", goto 45
        0, // on "exists", error
        0, // on "forall", error
        47, // on r#"\'[^\']+\'"#, goto 46
        48, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 47
        49, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 48
        50, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 49
        // State 93
        0, // on "(", error
        -51, // on ")", reduce `Value = "_" => ActionFn(30);`
        -51, // on ",", reduce `Value = "_" => ActionFn(30);`
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -51, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Value = "_" => ActionFn(30);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -51, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Value = "_" => ActionFn(30);`
        // State 94
        0, // on "(", error
        -14, // on ")", reduce `Atom = r#"\'[^\']+\'"# => ActionFn(31);`
        -14, // on ",", reduce `Atom = r#"\'[^\']+\'"# => ActionFn(31);`
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -14, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Atom = r#"\'[^\']+\'"# => ActionFn(31);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -14, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Atom = r#"\'[^\']+\'"# => ActionFn(31);`
        // State 95
        -44, // on "(", reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        -44, // on ")", reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        -44, // on ",", reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        -44, // on "_", reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        0, // on "exists", error
        0, // on "forall", error
        -44, // on r#"\'[^\']+\'"#, reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        -44, // on r#"[A-Za-z][A-Za-z0-9_]*"#, reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 96
        -38, // on "(", reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        -38, // on ")", reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        -38, // on ",", reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -38, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -38, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        // State 97
        -43, // on "(", reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        -43, // on ")", reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        -43, // on ",", reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        -43, // on "_", reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        0, // on "exists", error
        0, // on "forall", error
        -43, // on r#"\'[^\']+\'"#, reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        -43, // on r#"[A-Za-z][A-Za-z0-9_]*"#, reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 98
        45, // on "(", goto 44
        -10, // on ")", reduce `ApplicationBits = (BitOperator BitValue)+, BitOperator => ActionFn(80);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        46, // on "_", goto 45
        0, // on "exists", error
        0, // on "forall", error
        47, // on r#"\'[^\']+\'"#, goto 46
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        103, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 102
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 99
        0, // on "(", error
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        -50, // on ".", reduce `Value = "(", Application, ")" => ActionFn(29);`
        -50, // on ":-", reduce `Value = "(", Application, ")" => ActionFn(29);`
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -50, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Value = "(", Application, ")" => ActionFn(29);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -50, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Value = "(", Application, ")" => ActionFn(29);`
        // State 100
        0, // on "(", error
        -2, // on ")", reduce `(BitOperator BitValue)+ = BitOperator, BitValue => ActionFn(53);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -2, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `(BitOperator BitValue)+ = BitOperator, BitValue => ActionFn(53);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -2, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `(BitOperator BitValue)+ = BitOperator, BitValue => ActionFn(53);`
        // State 101
        0, // on "(", error
        -52, // on ")", reduce `Variable = Identifier => ActionFn(32);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -52, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Variable = Identifier => ActionFn(32);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -52, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Variable = Identifier => ActionFn(32);`
        // State 102
        0, // on "(", error
        -38, // on ")", reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -38, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -38, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        // State 103
        0, // on "(", error
        -11, // on ")", reduce `ApplicationBits = BitValue, (BitOperator BitValue)+ => ActionFn(81);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        48, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 47
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        50, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 49
        // State 104
        45, // on "(", goto 44
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        46, // on "_", goto 45
        0, // on "exists", error
        0, // on "forall", error
        47, // on r#"\'[^\']+\'"#, goto 46
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        103, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 102
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 105
        93, // on "(", goto 92
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        94, // on "_", goto 93
        0, // on "exists", error
        0, // on "forall", error
        95, // on r#"\'[^\']+\'"#, goto 94
        96, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 95
        97, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 96
        98, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 97
        // State 106
        0, // on "(", error
        135, // on ")", goto 134
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 107
        71, // on "(", goto 70
        0, // on ")", error
        -10, // on ",", reduce `ApplicationBits = (BitOperator BitValue)+, BitOperator => ActionFn(80);`
        0, // on "->", error
        -10, // on ".", reduce `ApplicationBits = (BitOperator BitValue)+, BitOperator => ActionFn(80);`
        0, // on ":-", error
        -10, // on ";", reduce `ApplicationBits = (BitOperator BitValue)+, BitOperator => ActionFn(80);`
        -10, // on "=>", reduce `ApplicationBits = (BitOperator BitValue)+, BitOperator => ActionFn(80);`
        72, // on "_", goto 71
        0, // on "exists", error
        0, // on "forall", error
        75, // on r#"\'[^\']+\'"#, goto 74
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        111, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 110
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 108
        0, // on "(", error
        0, // on ")", error
        -2, // on ",", reduce `(BitOperator BitValue)+ = BitOperator, BitValue => ActionFn(53);`
        0, // on "->", error
        -2, // on ".", reduce `(BitOperator BitValue)+ = BitOperator, BitValue => ActionFn(53);`
        0, // on ":-", error
        -2, // on ";", reduce `(BitOperator BitValue)+ = BitOperator, BitValue => ActionFn(53);`
        -2, // on "=>", reduce `(BitOperator BitValue)+ = BitOperator, BitValue => ActionFn(53);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -2, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `(BitOperator BitValue)+ = BitOperator, BitValue => ActionFn(53);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -2, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `(BitOperator BitValue)+ = BitOperator, BitValue => ActionFn(53);`
        // State 109
        0, // on "(", error
        0, // on ")", error
        -52, // on ",", reduce `Variable = Identifier => ActionFn(32);`
        0, // on "->", error
        -52, // on ".", reduce `Variable = Identifier => ActionFn(32);`
        0, // on ":-", error
        -52, // on ";", reduce `Variable = Identifier => ActionFn(32);`
        -52, // on "=>", reduce `Variable = Identifier => ActionFn(32);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -52, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Variable = Identifier => ActionFn(32);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -52, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Variable = Identifier => ActionFn(32);`
        // State 110
        0, // on "(", error
        0, // on ")", error
        -38, // on ",", reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        0, // on "->", error
        -38, // on ".", reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        0, // on ":-", error
        -38, // on ";", reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        -38, // on "=>", reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -38, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -38, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        // State 111
        0, // on "(", error
        0, // on ")", error
        -11, // on ",", reduce `ApplicationBits = BitValue, (BitOperator BitValue)+ => ActionFn(81);`
        0, // on "->", error
        -11, // on ".", reduce `ApplicationBits = BitValue, (BitOperator BitValue)+ => ActionFn(81);`
        0, // on ":-", error
        -11, // on ";", reduce `ApplicationBits = BitValue, (BitOperator BitValue)+ => ActionFn(81);`
        -11, // on "=>", reduce `ApplicationBits = BitValue, (BitOperator BitValue)+ => ActionFn(81);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        76, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 75
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        78, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 77
        // State 112
        71, // on "(", goto 70
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        72, // on "_", goto 71
        0, // on "exists", error
        0, // on "forall", error
        75, // on r#"\'[^\']+\'"#, goto 74
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        111, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 110
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 113
        -47, // on "(", reduce `Rule = Application, ":-", Fact<FactData>, "." => ActionFn(76);`
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        -47, // on "_", reduce `Rule = Application, ":-", Fact<FactData>, "." => ActionFn(76);`
        0, // on "exists", error
        0, // on "forall", error
        -47, // on r#"\'[^\']+\'"#, reduce `Rule = Application, ":-", Fact<FactData>, "." => ActionFn(76);`
        -47, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Rule = Application, ":-", Fact<FactData>, "." => ActionFn(76);`
        -47, // on r#"[A-Za-z][A-Za-z0-9_]*"#, reduce `Rule = Application, ":-", Fact<FactData>, "." => ActionFn(76);`
        -47, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Rule = Application, ":-", Fact<FactData>, "." => ActionFn(76);`
        // State 114
        71, // on "(", goto 70
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        72, // on "_", goto 71
        73, // on "exists", goto 72
        74, // on "forall", goto 73
        75, // on r#"\'[^\']+\'"#, goto 74
        76, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 75
        77, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 76
        78, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 77
        // State 115
        71, // on "(", goto 70
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        72, // on "_", goto 71
        73, // on "exists", goto 72
        74, // on "forall", goto 73
        75, // on r#"\'[^\']+\'"#, goto 74
        76, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 75
        77, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 76
        78, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 77
        // State 116
        71, // on "(", goto 70
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        72, // on "_", goto 71
        73, // on "exists", goto 72
        74, // on "forall", goto 73
        75, // on r#"\'[^\']+\'"#, goto 74
        76, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 75
        77, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 76
        78, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 77
        // State 117
        93, // on "(", goto 92
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        94, // on "_", goto 93
        0, // on "exists", error
        0, // on "forall", error
        95, // on r#"\'[^\']+\'"#, goto 94
        96, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 95
        97, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 96
        98, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 97
        // State 118
        0, // on "(", error
        144, // on ")", goto 143
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 119
        0, // on "(", error
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        147, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 146
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 120
        0, // on "(", error
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        147, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 146
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 121
        93, // on "(", goto 92
        -10, // on ")", reduce `ApplicationBits = (BitOperator BitValue)+, BitOperator => ActionFn(80);`
        -10, // on ",", reduce `ApplicationBits = (BitOperator BitValue)+, BitOperator => ActionFn(80);`
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        94, // on "_", goto 93
        0, // on "exists", error
        0, // on "forall", error
        95, // on r#"\'[^\']+\'"#, goto 94
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        126, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 125
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 122
        0, // on "(", error
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        -13, // on ".", reduce `ApplicationBits = Identifier, "(", BitApplications, ")" => ActionFn(67);`
        -13, // on ":-", reduce `ApplicationBits = Identifier, "(", BitApplications, ")" => ActionFn(67);`
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 123
        0, // on "(", error
        -2, // on ")", reduce `(BitOperator BitValue)+ = BitOperator, BitValue => ActionFn(53);`
        -2, // on ",", reduce `(BitOperator BitValue)+ = BitOperator, BitValue => ActionFn(53);`
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -2, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `(BitOperator BitValue)+ = BitOperator, BitValue => ActionFn(53);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -2, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `(BitOperator BitValue)+ = BitOperator, BitValue => ActionFn(53);`
        // State 124
        0, // on "(", error
        -52, // on ")", reduce `Variable = Identifier => ActionFn(32);`
        -52, // on ",", reduce `Variable = Identifier => ActionFn(32);`
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -52, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Variable = Identifier => ActionFn(32);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -52, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Variable = Identifier => ActionFn(32);`
        // State 125
        0, // on "(", error
        -38, // on ")", reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        -38, // on ",", reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -38, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -38, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        // State 126
        0, // on "(", error
        -11, // on ")", reduce `ApplicationBits = BitValue, (BitOperator BitValue)+ => ActionFn(81);`
        -11, // on ",", reduce `ApplicationBits = BitValue, (BitOperator BitValue)+ => ActionFn(81);`
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        96, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 95
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        98, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 97
        // State 127
        93, // on "(", goto 92
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        94, // on "_", goto 93
        0, // on "exists", error
        0, // on "forall", error
        95, // on r#"\'[^\']+\'"#, goto 94
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        126, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 125
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 128
        93, // on "(", goto 92
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        94, // on "_", goto 93
        0, // on "exists", error
        0, // on "forall", error
        95, // on r#"\'[^\']+\'"#, goto 94
        96, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 95
        97, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 96
        98, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 97
        // State 129
        93, // on "(", goto 92
        -54, // on ")", reduce `Vec1<BitApplication> = Vec1<BitApplication>, "," => ActionFn(35);`
        -54, // on ",", reduce `Vec1<BitApplication> = Vec1<BitApplication>, "," => ActionFn(35);`
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        94, // on "_", goto 93
        0, // on "exists", error
        0, // on "forall", error
        95, // on r#"\'[^\']+\'"#, goto 94
        96, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 95
        97, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 96
        98, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 97
        // State 130
        0, // on "(", error
        153, // on ")", goto 152
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 131
        0, // on "(", error
        -3, // on ")", reduce `(BitOperator BitValue)+ = (BitOperator BitValue)+, BitOperator, BitValue => ActionFn(54);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -3, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `(BitOperator BitValue)+ = (BitOperator BitValue)+, BitOperator, BitValue => ActionFn(54);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -3, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `(BitOperator BitValue)+ = (BitOperator BitValue)+, BitOperator, BitValue => ActionFn(54);`
        // State 132
        45, // on "(", goto 44
        -9, // on ")", reduce `ApplicationBits = BitValue, (BitOperator BitValue)+, BitOperator => ActionFn(79);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        46, // on "_", goto 45
        0, // on "exists", error
        0, // on "forall", error
        47, // on r#"\'[^\']+\'"#, goto 46
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        103, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 102
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 133
        0, // on "(", error
        154, // on ")", goto 153
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 134
        0, // on "(", error
        -50, // on ")", reduce `Value = "(", Application, ")" => ActionFn(29);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -50, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Value = "(", Application, ")" => ActionFn(29);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -50, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Value = "(", Application, ")" => ActionFn(29);`
        // State 135
        0, // on "(", error
        0, // on ")", error
        -3, // on ",", reduce `(BitOperator BitValue)+ = (BitOperator BitValue)+, BitOperator, BitValue => ActionFn(54);`
        0, // on "->", error
        -3, // on ".", reduce `(BitOperator BitValue)+ = (BitOperator BitValue)+, BitOperator, BitValue => ActionFn(54);`
        0, // on ":-", error
        -3, // on ";", reduce `(BitOperator BitValue)+ = (BitOperator BitValue)+, BitOperator, BitValue => ActionFn(54);`
        -3, // on "=>", reduce `(BitOperator BitValue)+ = (BitOperator BitValue)+, BitOperator, BitValue => ActionFn(54);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -3, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `(BitOperator BitValue)+ = (BitOperator BitValue)+, BitOperator, BitValue => ActionFn(54);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -3, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `(BitOperator BitValue)+ = (BitOperator BitValue)+, BitOperator, BitValue => ActionFn(54);`
        // State 136
        71, // on "(", goto 70
        0, // on ")", error
        -9, // on ",", reduce `ApplicationBits = BitValue, (BitOperator BitValue)+, BitOperator => ActionFn(79);`
        0, // on "->", error
        -9, // on ".", reduce `ApplicationBits = BitValue, (BitOperator BitValue)+, BitOperator => ActionFn(79);`
        0, // on ":-", error
        -9, // on ";", reduce `ApplicationBits = BitValue, (BitOperator BitValue)+, BitOperator => ActionFn(79);`
        -9, // on "=>", reduce `ApplicationBits = BitValue, (BitOperator BitValue)+, BitOperator => ActionFn(79);`
        72, // on "_", goto 71
        0, // on "exists", error
        0, // on "forall", error
        75, // on r#"\'[^\']+\'"#, goto 74
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        111, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 110
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 137
        0, // on "(", error
        0, // on ")", error
        117, // on ",", goto 116
        0, // on "->", error
        -30, // on ".", reduce `FactDataAnd = Fact<FactDataAnd>, ";", Fact<FactDataOr> => ActionFn(7);`
        0, // on ":-", error
        -30, // on ";", reduce `FactDataAnd = Fact<FactDataAnd>, ";", Fact<FactDataOr> => ActionFn(7);`
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 138
        0, // on "(", error
        0, // on ")", error
        -27, // on ",", reduce `Fact<FactDataOr> = FactDataOr => ActionFn(75);`
        0, // on "->", error
        -27, // on ".", reduce `Fact<FactDataOr> = FactDataOr => ActionFn(75);`
        0, // on ":-", error
        -27, // on ";", reduce `Fact<FactDataOr> = FactDataOr => ActionFn(75);`
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 139
        0, // on "(", error
        0, // on ")", error
        -33, // on ",", reduce `FactDataFunc = Fact<FactDataApply>, "=>", Fact<FactDataFunc> => ActionFn(11);`
        0, // on "->", error
        -33, // on ".", reduce `FactDataFunc = Fact<FactDataApply>, "=>", Fact<FactDataFunc> => ActionFn(11);`
        0, // on ":-", error
        -33, // on ";", reduce `FactDataFunc = Fact<FactDataApply>, "=>", Fact<FactDataFunc> => ActionFn(11);`
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 140
        0, // on "(", error
        0, // on ")", error
        -26, // on ",", reduce `Fact<FactDataFunc> = FactDataFunc => ActionFn(74);`
        0, // on "->", error
        -26, // on ".", reduce `Fact<FactDataFunc> = FactDataFunc => ActionFn(74);`
        0, // on ":-", error
        -26, // on ";", reduce `Fact<FactDataFunc> = FactDataFunc => ActionFn(74);`
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 141
        0, // on "(", error
        0, // on ")", error
        -37, // on ",", reduce `FactDataOr = Fact<FactDataOr>, ",", Fact<FactDataFunc> => ActionFn(9);`
        0, // on "->", error
        -37, // on ".", reduce `FactDataOr = Fact<FactDataOr>, ",", Fact<FactDataFunc> => ActionFn(9);`
        0, // on ":-", error
        -37, // on ";", reduce `FactDataOr = Fact<FactDataOr>, ",", Fact<FactDataFunc> => ActionFn(9);`
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 142
        0, // on "(", error
        155, // on ")", goto 154
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 143
        0, // on "(", error
        0, // on ")", error
        -50, // on ",", reduce `Value = "(", Application, ")" => ActionFn(29);`
        0, // on "->", error
        -50, // on ".", reduce `Value = "(", Application, ")" => ActionFn(29);`
        0, // on ":-", error
        -50, // on ";", reduce `Value = "(", Application, ")" => ActionFn(29);`
        -50, // on "=>", reduce `Value = "(", Application, ")" => ActionFn(29);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -50, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Value = "(", Application, ")" => ActionFn(29);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -50, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Value = "(", Application, ")" => ActionFn(29);`
        // State 144
        0, // on "(", error
        0, // on ")", error
        0, // on ",", error
        -52, // on "->", reduce `Variable = Identifier => ActionFn(32);`
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 145
        0, // on "(", error
        0, // on ")", error
        0, // on ",", error
        156, // on "->", goto 155
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 146
        0, // on "(", error
        0, // on ")", error
        0, // on ",", error
        -38, // on "->", reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 147
        0, // on "(", error
        0, // on ")", error
        0, // on ",", error
        157, // on "->", goto 156
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 148
        0, // on "(", error
        -3, // on ")", reduce `(BitOperator BitValue)+ = (BitOperator BitValue)+, BitOperator, BitValue => ActionFn(54);`
        -3, // on ",", reduce `(BitOperator BitValue)+ = (BitOperator BitValue)+, BitOperator, BitValue => ActionFn(54);`
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -3, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `(BitOperator BitValue)+ = (BitOperator BitValue)+, BitOperator, BitValue => ActionFn(54);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -3, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `(BitOperator BitValue)+ = (BitOperator BitValue)+, BitOperator, BitValue => ActionFn(54);`
        // State 149
        93, // on "(", goto 92
        -9, // on ")", reduce `ApplicationBits = BitValue, (BitOperator BitValue)+, BitOperator => ActionFn(79);`
        -9, // on ",", reduce `ApplicationBits = BitValue, (BitOperator BitValue)+, BitOperator => ActionFn(79);`
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        94, // on "_", goto 93
        0, // on "exists", error
        0, // on "forall", error
        95, // on r#"\'[^\']+\'"#, goto 94
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        126, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 125
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 150
        0, // on "(", error
        158, // on ")", goto 157
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 151
        0, // on "(", error
        -55, // on ")", reduce `Vec1<BitApplication> = Vec1<BitApplication>, ",", BitApplication => ActionFn(36);`
        -55, // on ",", reduce `Vec1<BitApplication> = Vec1<BitApplication>, ",", BitApplication => ActionFn(36);`
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 152
        0, // on "(", error
        -50, // on ")", reduce `Value = "(", Application, ")" => ActionFn(29);`
        -50, // on ",", reduce `Value = "(", Application, ")" => ActionFn(29);`
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -50, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Value = "(", Application, ")" => ActionFn(29);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -50, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Value = "(", Application, ")" => ActionFn(29);`
        // State 153
        0, // on "(", error
        -13, // on ")", reduce `ApplicationBits = Identifier, "(", BitApplications, ")" => ActionFn(67);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 154
        0, // on "(", error
        0, // on ")", error
        -13, // on ",", reduce `ApplicationBits = Identifier, "(", BitApplications, ")" => ActionFn(67);`
        0, // on "->", error
        -13, // on ".", reduce `ApplicationBits = Identifier, "(", BitApplications, ")" => ActionFn(67);`
        0, // on ":-", error
        -13, // on ";", reduce `ApplicationBits = Identifier, "(", BitApplications, ")" => ActionFn(67);`
        -13, // on "=>", reduce `ApplicationBits = Identifier, "(", BitApplications, ")" => ActionFn(67);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 155
        173, // on "(", goto 172
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        174, // on "_", goto 173
        175, // on "exists", goto 174
        176, // on "forall", goto 175
        177, // on r#"\'[^\']+\'"#, goto 176
        178, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 177
        179, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 178
        180, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 179
        // State 156
        173, // on "(", goto 172
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        174, // on "_", goto 173
        175, // on "exists", goto 174
        176, // on "forall", goto 175
        177, // on r#"\'[^\']+\'"#, goto 176
        178, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 177
        179, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 178
        180, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 179
        // State 157
        0, // on "(", error
        -13, // on ")", reduce `ApplicationBits = Identifier, "(", BitApplications, ")" => ActionFn(67);`
        -13, // on ",", reduce `ApplicationBits = Identifier, "(", BitApplications, ")" => ActionFn(67);`
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 158
        0, // on "(", error
        -12, // on ")", reduce `ApplicationBits = (BitOperator BitValue)+ => ActionFn(82);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        -12, // on "=>", reduce `ApplicationBits = (BitOperator BitValue)+ => ActionFn(82);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        178, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 177
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        180, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 179
        // State 159
        0, // on "(", error
        -31, // on ")", reduce `FactDataApply = Application => ActionFn(14);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        -31, // on "=>", reduce `FactDataApply = Application => ActionFn(14);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 160
        0, // on "(", error
        -6, // on ")", reduce `Application = ApplicationBits => ActionFn(66);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        -6, // on "=>", reduce `Application = ApplicationBits => ActionFn(66);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 161
        0, // on "(", error
        -48, // on ")", reduce `Value = Atom => ActionFn(27);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        -48, // on "=>", reduce `Value = Atom => ActionFn(27);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -48, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Value = Atom => ActionFn(27);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -48, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Value = Atom => ActionFn(27);`
        // State 162
        173, // on "(", goto 172
        -8, // on ")", reduce `ApplicationBits = BitOperator => ActionFn(17);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        -8, // on "=>", reduce `ApplicationBits = BitOperator => ActionFn(17);`
        174, // on "_", goto 173
        0, // on "exists", error
        0, // on "forall", error
        177, // on r#"\'[^\']+\'"#, goto 176
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        185, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 184
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 163
        0, // on "(", error
        -7, // on ")", reduce `ApplicationBits = BitValue => ActionFn(16);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        -7, // on "=>", reduce `ApplicationBits = BitValue => ActionFn(16);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        31, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 30
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        32, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 31
        // State 164
        0, // on "(", error
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        188, // on "=>", goto 187
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 165
        0, // on "(", error
        189, // on ")", goto 188
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 166
        0, // on "(", error
        -32, // on ")", reduce `FactDataFunc = FactDataApply => ActionFn(10);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        -25, // on "=>", reduce `Fact<FactDataApply> = FactDataApply => ActionFn(73);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 167
        0, // on "(", error
        -26, // on ")", reduce `Fact<FactDataFunc> = FactDataFunc => ActionFn(74);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 168
        190, // on "(", goto 189
        -52, // on ")", reduce `Variable = Identifier => ActionFn(32);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        -52, // on "=>", reduce `Variable = Identifier => ActionFn(32);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -52, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Variable = Identifier => ActionFn(32);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -52, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Variable = Identifier => ActionFn(32);`
        // State 169
        -17, // on "(", reduce `BitOperator = Operator => ActionFn(69);`
        -17, // on ")", reduce `BitOperator = Operator => ActionFn(69);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        -17, // on "=>", reduce `BitOperator = Operator => ActionFn(69);`
        -17, // on "_", reduce `BitOperator = Operator => ActionFn(69);`
        0, // on "exists", error
        0, // on "forall", error
        -17, // on r#"\'[^\']+\'"#, reduce `BitOperator = Operator => ActionFn(69);`
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        -17, // on r#"[A-Za-z][A-Za-z0-9_]*"#, reduce `BitOperator = Operator => ActionFn(69);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 170
        0, // on "(", error
        -20, // on ")", reduce `BitValue = Value => ActionFn(70);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        -20, // on "=>", reduce `BitValue = Value => ActionFn(70);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -20, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `BitValue = Value => ActionFn(70);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -20, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `BitValue = Value => ActionFn(70);`
        // State 171
        0, // on "(", error
        -49, // on ")", reduce `Value = Variable => ActionFn(28);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        -49, // on "=>", reduce `Value = Variable => ActionFn(28);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -49, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Value = Variable => ActionFn(28);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -49, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Value = Variable => ActionFn(28);`
        // State 172
        45, // on "(", goto 44
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        46, // on "_", goto 45
        0, // on "exists", error
        0, // on "forall", error
        47, // on r#"\'[^\']+\'"#, goto 46
        48, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 47
        49, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 48
        50, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 49
        // State 173
        0, // on "(", error
        -51, // on ")", reduce `Value = "_" => ActionFn(30);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        -51, // on "=>", reduce `Value = "_" => ActionFn(30);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -51, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Value = "_" => ActionFn(30);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -51, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Value = "_" => ActionFn(30);`
        // State 174
        192, // on "(", goto 191
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 175
        193, // on "(", goto 192
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 176
        0, // on "(", error
        -14, // on ")", reduce `Atom = r#"\'[^\']+\'"# => ActionFn(31);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        -14, // on "=>", reduce `Atom = r#"\'[^\']+\'"# => ActionFn(31);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -14, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Atom = r#"\'[^\']+\'"# => ActionFn(31);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -14, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Atom = r#"\'[^\']+\'"# => ActionFn(31);`
        // State 177
        -44, // on "(", reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        -44, // on ")", reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        -44, // on "=>", reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        -44, // on "_", reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        0, // on "exists", error
        0, // on "forall", error
        -44, // on r#"\'[^\']+\'"#, reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        -44, // on r#"[A-Za-z][A-Za-z0-9_]*"#, reduce `Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 178
        -38, // on "(", reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        -38, // on ")", reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        -38, // on "=>", reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -38, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -38, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        // State 179
        -43, // on "(", reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        -43, // on ")", reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        -43, // on "=>", reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        -43, // on "_", reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        0, // on "exists", error
        0, // on "forall", error
        -43, // on r#"\'[^\']+\'"#, reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        -43, // on r#"[A-Za-z][A-Za-z0-9_]*"#, reduce `Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 180
        0, // on "(", error
        194, // on ")", goto 193
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 181
        173, // on "(", goto 172
        -10, // on ")", reduce `ApplicationBits = (BitOperator BitValue)+, BitOperator => ActionFn(80);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        -10, // on "=>", reduce `ApplicationBits = (BitOperator BitValue)+, BitOperator => ActionFn(80);`
        174, // on "_", goto 173
        0, // on "exists", error
        0, // on "forall", error
        177, // on r#"\'[^\']+\'"#, goto 176
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        185, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 184
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 182
        0, // on "(", error
        -2, // on ")", reduce `(BitOperator BitValue)+ = BitOperator, BitValue => ActionFn(53);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        -2, // on "=>", reduce `(BitOperator BitValue)+ = BitOperator, BitValue => ActionFn(53);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -2, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `(BitOperator BitValue)+ = BitOperator, BitValue => ActionFn(53);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -2, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `(BitOperator BitValue)+ = BitOperator, BitValue => ActionFn(53);`
        // State 183
        0, // on "(", error
        -52, // on ")", reduce `Variable = Identifier => ActionFn(32);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        -52, // on "=>", reduce `Variable = Identifier => ActionFn(32);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -52, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Variable = Identifier => ActionFn(32);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -52, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Variable = Identifier => ActionFn(32);`
        // State 184
        0, // on "(", error
        -38, // on ")", reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        -38, // on "=>", reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -38, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -38, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);`
        // State 185
        0, // on "(", error
        -11, // on ")", reduce `ApplicationBits = BitValue, (BitOperator BitValue)+ => ActionFn(81);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        -11, // on "=>", reduce `ApplicationBits = BitValue, (BitOperator BitValue)+ => ActionFn(81);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        178, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 177
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        180, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 179
        // State 186
        173, // on "(", goto 172
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        174, // on "_", goto 173
        0, // on "exists", error
        0, // on "forall", error
        177, // on r#"\'[^\']+\'"#, goto 176
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        185, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 184
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 187
        173, // on "(", goto 172
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        174, // on "_", goto 173
        175, // on "exists", goto 174
        176, // on "forall", goto 175
        177, // on r#"\'[^\']+\'"#, goto 176
        178, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 177
        179, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 178
        180, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 179
        // State 188
        0, // on "(", error
        0, // on ")", error
        -34, // on ",", reduce `FactDataFunc = "exists", "(", Variable, "->", Fact<FactDataFunc>, ")" => ActionFn(12);`
        0, // on "->", error
        -34, // on ".", reduce `FactDataFunc = "exists", "(", Variable, "->", Fact<FactDataFunc>, ")" => ActionFn(12);`
        0, // on ":-", error
        -34, // on ";", reduce `FactDataFunc = "exists", "(", Variable, "->", Fact<FactDataFunc>, ")" => ActionFn(12);`
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 189
        93, // on "(", goto 92
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        94, // on "_", goto 93
        0, // on "exists", error
        0, // on "forall", error
        95, // on r#"\'[^\']+\'"#, goto 94
        96, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 95
        97, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 96
        98, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 97
        // State 190
        0, // on "(", error
        199, // on ")", goto 198
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 191
        0, // on "(", error
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        147, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 146
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 192
        0, // on "(", error
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        147, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 146
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 193
        0, // on "(", error
        0, // on ")", error
        -35, // on ",", reduce `FactDataFunc = "forall", "(", Variable, "->", Fact<FactDataFunc>, ")" => ActionFn(13);`
        0, // on "->", error
        -35, // on ".", reduce `FactDataFunc = "forall", "(", Variable, "->", Fact<FactDataFunc>, ")" => ActionFn(13);`
        0, // on ":-", error
        -35, // on ";", reduce `FactDataFunc = "forall", "(", Variable, "->", Fact<FactDataFunc>, ")" => ActionFn(13);`
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 194
        0, // on "(", error
        -3, // on ")", reduce `(BitOperator BitValue)+ = (BitOperator BitValue)+, BitOperator, BitValue => ActionFn(54);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        -3, // on "=>", reduce `(BitOperator BitValue)+ = (BitOperator BitValue)+, BitOperator, BitValue => ActionFn(54);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -3, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `(BitOperator BitValue)+ = (BitOperator BitValue)+, BitOperator, BitValue => ActionFn(54);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -3, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `(BitOperator BitValue)+ = (BitOperator BitValue)+, BitOperator, BitValue => ActionFn(54);`
        // State 195
        173, // on "(", goto 172
        -9, // on ")", reduce `ApplicationBits = BitValue, (BitOperator BitValue)+, BitOperator => ActionFn(79);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        -9, // on "=>", reduce `ApplicationBits = BitValue, (BitOperator BitValue)+, BitOperator => ActionFn(79);`
        174, // on "_", goto 173
        0, // on "exists", error
        0, // on "forall", error
        177, // on r#"\'[^\']+\'"#, goto 176
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        185, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 184
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 196
        0, // on "(", error
        -33, // on ")", reduce `FactDataFunc = Fact<FactDataApply>, "=>", Fact<FactDataFunc> => ActionFn(11);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 197
        0, // on "(", error
        202, // on ")", goto 201
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 198
        0, // on "(", error
        -50, // on ")", reduce `Value = "(", Application, ")" => ActionFn(29);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        -50, // on "=>", reduce `Value = "(", Application, ")" => ActionFn(29);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        -50, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, reduce `Value = "(", Application, ")" => ActionFn(29);`
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        -50, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, reduce `Value = "(", Application, ")" => ActionFn(29);`
        // State 199
        0, // on "(", error
        0, // on ")", error
        0, // on ",", error
        203, // on "->", goto 202
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 200
        0, // on "(", error
        0, // on ")", error
        0, // on ",", error
        204, // on "->", goto 203
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 201
        0, // on "(", error
        -13, // on ")", reduce `ApplicationBits = Identifier, "(", BitApplications, ")" => ActionFn(67);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        -13, // on "=>", reduce `ApplicationBits = Identifier, "(", BitApplications, ")" => ActionFn(67);`
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 202
        173, // on "(", goto 172
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        174, // on "_", goto 173
        175, // on "exists", goto 174
        176, // on "forall", goto 175
        177, // on r#"\'[^\']+\'"#, goto 176
        178, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 177
        179, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 178
        180, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 179
        // State 203
        173, // on "(", goto 172
        0, // on ")", error
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        174, // on "_", goto 173
        175, // on "exists", goto 174
        176, // on "forall", goto 175
        177, // on r#"\'[^\']+\'"#, goto 176
        178, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, goto 177
        179, // on r#"[A-Za-z][A-Za-z0-9_]*"#, goto 178
        180, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, goto 179
        // State 204
        0, // on "(", error
        207, // on ")", goto 206
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 205
        0, // on "(", error
        208, // on ")", goto 207
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 206
        0, // on "(", error
        -34, // on ")", reduce `FactDataFunc = "exists", "(", Variable, "->", Fact<FactDataFunc>, ")" => ActionFn(12);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
        // State 207
        0, // on "(", error
        -35, // on ")", reduce `FactDataFunc = "forall", "(", Variable, "->", Fact<FactDataFunc>, ")" => ActionFn(13);`
        0, // on ",", error
        0, // on "->", error
        0, // on ".", error
        0, // on ":-", error
        0, // on ";", error
        0, // on "=>", error
        0, // on "_", error
        0, // on "exists", error
        0, // on "forall", error
        0, // on r#"\'[^\']+\'"#, error
        0, // on r#"[-|!@#$%^&*=+/:?~<>]+"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*"#, error
        0, // on r#"[A-Za-z][A-Za-z0-9_]*:"#, error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -41, // on EOF, reduce `Item+ = Item => ActionFn(51);`
        -46, // on EOF, reduce `Program = Item+ => ActionFn(1);`
        0, // on EOF, error
        -56, // on EOF, reduce `__Program = Program => ActionFn(0);`
        -40, // on EOF, reduce `Item = Rule => ActionFn(3);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -39, // on EOF, reduce `Item = Application, "." => ActionFn(2);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -42, // on EOF, reduce `Item+ = Item+, Item => ActionFn(52);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -47, // on EOF, reduce `Rule = Application, ":-", Fact<FactData>, "." => ActionFn(76);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, // on (BitOperator BitValue), error
        2, // on (BitOperator BitValue)+, goto 1
        0, // on @L, error
        0, // on @R, error
        3, // on Application, goto 2
        4, // on ApplicationBits, goto 3
        5, // on Atom, goto 4
        0, // on BitApplication, error
        0, // on BitApplications, error
        6, // on BitOperator, goto 5
        0, // on BitOperator?, error
        7, // on BitValue, goto 6
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        8, // on Identifier, goto 7
        9, // on Item, goto 8
        10, // on Item+, goto 9
        11, // on Operator, goto 10
        0, // on OperatorValue, error
        12, // on Program, goto 11
        13, // on Rule, goto 12
        14, // on Value, goto 13
        15, // on Variable, goto 14
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 1
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        22, // on BitOperator, goto 21
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        11, // on Operator, goto 10
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 2
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 3
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 4
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 5
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        5, // on Atom, goto 4
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        25, // on BitValue, goto 24
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        26, // on Identifier, goto 25
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        14, // on Value, goto 13
        15, // on Variable, goto 14
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 6
        0, // on (BitOperator BitValue), error
        28, // on (BitOperator BitValue)+, goto 27
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        29, // on BitOperator, goto 28
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        30, // on Operator, goto 29
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 7
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 8
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 9
        0, // on (BitOperator BitValue), error
        2, // on (BitOperator BitValue)+, goto 1
        0, // on @L, error
        0, // on @R, error
        3, // on Application, goto 2
        4, // on ApplicationBits, goto 3
        5, // on Atom, goto 4
        0, // on BitApplication, error
        0, // on BitApplications, error
        6, // on BitOperator, goto 5
        0, // on BitOperator?, error
        7, // on BitValue, goto 6
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        8, // on Identifier, goto 7
        34, // on Item, goto 33
        0, // on Item+, error
        11, // on Operator, goto 10
        0, // on OperatorValue, error
        0, // on Program, error
        13, // on Rule, goto 12
        14, // on Value, goto 13
        15, // on Variable, goto 14
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 10
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 11
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 12
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 13
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 14
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 15
        0, // on (BitOperator BitValue), error
        35, // on (BitOperator BitValue)+, goto 34
        0, // on @L, error
        0, // on @R, error
        36, // on Application, goto 35
        37, // on ApplicationBits, goto 36
        38, // on Atom, goto 37
        0, // on BitApplication, error
        0, // on BitApplications, error
        39, // on BitOperator, goto 38
        0, // on BitOperator?, error
        40, // on BitValue, goto 39
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        41, // on Identifier, goto 40
        0, // on Item, error
        0, // on Item+, error
        42, // on Operator, goto 41
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        43, // on Value, goto 42
        44, // on Variable, goto 43
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 16
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 17
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 18
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 19
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 20
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 21
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        5, // on Atom, goto 4
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        51, // on BitValue, goto 50
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        26, // on Identifier, goto 25
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        14, // on Value, goto 13
        15, // on Variable, goto 14
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 22
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 23
        0, // on (BitOperator BitValue), error
        52, // on (BitOperator BitValue)+, goto 51
        0, // on @L, error
        0, // on @R, error
        53, // on Application, goto 52
        54, // on ApplicationBits, goto 53
        55, // on Atom, goto 54
        0, // on BitApplication, error
        0, // on BitApplications, error
        56, // on BitOperator, goto 55
        0, // on BitOperator?, error
        57, // on BitValue, goto 56
        0, // on BitValue?, error
        58, // on Fact<FactData>, goto 57
        59, // on Fact<FactDataAnd>, goto 58
        60, // on Fact<FactDataApply>, goto 59
        0, // on Fact<FactDataFunc>, error
        61, // on Fact<FactDataOr>, goto 60
        62, // on FactData, goto 61
        63, // on FactDataAnd, goto 62
        64, // on FactDataApply, goto 63
        65, // on FactDataFunc, goto 64
        66, // on FactDataOr, goto 65
        67, // on Identifier, goto 66
        0, // on Item, error
        0, // on Item+, error
        68, // on Operator, goto 67
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        69, // on Value, goto 68
        70, // on Variable, goto 69
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 24
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 25
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 26
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 27
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        79, // on BitOperator, goto 78
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        11, // on Operator, goto 10
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 28
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        5, // on Atom, goto 4
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        25, // on BitValue, goto 24
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        26, // on Identifier, goto 25
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        14, // on Value, goto 13
        15, // on Variable, goto 14
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 29
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 30
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 31
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 32
        0, // on (BitOperator BitValue), error
        80, // on (BitOperator BitValue)+, goto 79
        0, // on @L, error
        0, // on @R, error
        81, // on Application, goto 80
        82, // on ApplicationBits, goto 81
        83, // on Atom, goto 82
        84, // on BitApplication, goto 83
        85, // on BitApplications, goto 84
        86, // on BitOperator, goto 85
        0, // on BitOperator?, error
        87, // on BitValue, goto 86
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        88, // on Identifier, goto 87
        0, // on Item, error
        0, // on Item+, error
        89, // on Operator, goto 88
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        90, // on Value, goto 89
        91, // on Variable, goto 90
        92, // on Vec1<BitApplication>, goto 91
        0, // on __Program, error
        // State 33
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 34
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        99, // on BitOperator, goto 98
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        42, // on Operator, goto 41
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 35
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 36
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 37
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 38
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        38, // on Atom, goto 37
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        101, // on BitValue, goto 100
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        102, // on Identifier, goto 101
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        43, // on Value, goto 42
        44, // on Variable, goto 43
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 39
        0, // on (BitOperator BitValue), error
        104, // on (BitOperator BitValue)+, goto 103
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        105, // on BitOperator, goto 104
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        30, // on Operator, goto 29
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 40
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 41
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 42
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 43
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 44
        0, // on (BitOperator BitValue), error
        35, // on (BitOperator BitValue)+, goto 34
        0, // on @L, error
        0, // on @R, error
        107, // on Application, goto 106
        37, // on ApplicationBits, goto 36
        38, // on Atom, goto 37
        0, // on BitApplication, error
        0, // on BitApplications, error
        39, // on BitOperator, goto 38
        0, // on BitOperator?, error
        40, // on BitValue, goto 39
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        41, // on Identifier, goto 40
        0, // on Item, error
        0, // on Item+, error
        42, // on Operator, goto 41
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        43, // on Value, goto 42
        44, // on Variable, goto 43
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 45
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 46
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 47
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 48
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 49
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 50
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 51
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        108, // on BitOperator, goto 107
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        68, // on Operator, goto 67
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 52
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 53
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 54
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 55
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        55, // on Atom, goto 54
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        109, // on BitValue, goto 108
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        110, // on Identifier, goto 109
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        69, // on Value, goto 68
        70, // on Variable, goto 69
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 56
        0, // on (BitOperator BitValue), error
        112, // on (BitOperator BitValue)+, goto 111
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        113, // on BitOperator, goto 112
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        30, // on Operator, goto 29
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 57
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 58
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 59
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 60
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 61
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 62
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 63
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 64
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 65
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 66
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 67
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 68
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 69
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 70
        0, // on (BitOperator BitValue), error
        35, // on (BitOperator BitValue)+, goto 34
        0, // on @L, error
        0, // on @R, error
        119, // on Application, goto 118
        37, // on ApplicationBits, goto 36
        38, // on Atom, goto 37
        0, // on BitApplication, error
        0, // on BitApplications, error
        39, // on BitOperator, goto 38
        0, // on BitOperator?, error
        40, // on BitValue, goto 39
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        41, // on Identifier, goto 40
        0, // on Item, error
        0, // on Item+, error
        42, // on Operator, goto 41
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        43, // on Value, goto 42
        44, // on Variable, goto 43
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 71
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 72
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 73
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 74
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 75
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 76
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 77
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 78
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        5, // on Atom, goto 4
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        51, // on BitValue, goto 50
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        26, // on Identifier, goto 25
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        14, // on Value, goto 13
        15, // on Variable, goto 14
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 79
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        122, // on BitOperator, goto 121
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        89, // on Operator, goto 88
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 80
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 81
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 82
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 83
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 84
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 85
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        83, // on Atom, goto 82
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        124, // on BitValue, goto 123
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        125, // on Identifier, goto 124
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        90, // on Value, goto 89
        91, // on Variable, goto 90
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 86
        0, // on (BitOperator BitValue), error
        127, // on (BitOperator BitValue)+, goto 126
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        128, // on BitOperator, goto 127
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        30, // on Operator, goto 29
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 87
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 88
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 89
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 90
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 91
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 92
        0, // on (BitOperator BitValue), error
        35, // on (BitOperator BitValue)+, goto 34
        0, // on @L, error
        0, // on @R, error
        131, // on Application, goto 130
        37, // on ApplicationBits, goto 36
        38, // on Atom, goto 37
        0, // on BitApplication, error
        0, // on BitApplications, error
        39, // on BitOperator, goto 38
        0, // on BitOperator?, error
        40, // on BitValue, goto 39
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        41, // on Identifier, goto 40
        0, // on Item, error
        0, // on Item+, error
        42, // on Operator, goto 41
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        43, // on Value, goto 42
        44, // on Variable, goto 43
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 93
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 94
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 95
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 96
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 97
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 98
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        38, // on Atom, goto 37
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        132, // on BitValue, goto 131
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        102, // on Identifier, goto 101
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        43, // on Value, goto 42
        44, // on Variable, goto 43
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 99
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 100
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 101
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 102
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 103
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        133, // on BitOperator, goto 132
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        42, // on Operator, goto 41
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 104
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        38, // on Atom, goto 37
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        101, // on BitValue, goto 100
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        102, // on Identifier, goto 101
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        43, // on Value, goto 42
        44, // on Variable, goto 43
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 105
        0, // on (BitOperator BitValue), error
        80, // on (BitOperator BitValue)+, goto 79
        0, // on @L, error
        0, // on @R, error
        81, // on Application, goto 80
        82, // on ApplicationBits, goto 81
        83, // on Atom, goto 82
        84, // on BitApplication, goto 83
        134, // on BitApplications, goto 133
        86, // on BitOperator, goto 85
        0, // on BitOperator?, error
        87, // on BitValue, goto 86
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        88, // on Identifier, goto 87
        0, // on Item, error
        0, // on Item+, error
        89, // on Operator, goto 88
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        90, // on Value, goto 89
        91, // on Variable, goto 90
        92, // on Vec1<BitApplication>, goto 91
        0, // on __Program, error
        // State 106
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 107
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        55, // on Atom, goto 54
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        136, // on BitValue, goto 135
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        110, // on Identifier, goto 109
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        69, // on Value, goto 68
        70, // on Variable, goto 69
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 108
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 109
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 110
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 111
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        137, // on BitOperator, goto 136
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        68, // on Operator, goto 67
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 112
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        55, // on Atom, goto 54
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        109, // on BitValue, goto 108
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        110, // on Identifier, goto 109
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        69, // on Value, goto 68
        70, // on Variable, goto 69
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 113
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 114
        0, // on (BitOperator BitValue), error
        52, // on (BitOperator BitValue)+, goto 51
        0, // on @L, error
        0, // on @R, error
        53, // on Application, goto 52
        54, // on ApplicationBits, goto 53
        55, // on Atom, goto 54
        0, // on BitApplication, error
        0, // on BitApplications, error
        56, // on BitOperator, goto 55
        0, // on BitOperator?, error
        57, // on BitValue, goto 56
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        60, // on Fact<FactDataApply>, goto 59
        0, // on Fact<FactDataFunc>, error
        138, // on Fact<FactDataOr>, goto 137
        0, // on FactData, error
        0, // on FactDataAnd, error
        64, // on FactDataApply, goto 63
        65, // on FactDataFunc, goto 64
        139, // on FactDataOr, goto 138
        67, // on Identifier, goto 66
        0, // on Item, error
        0, // on Item+, error
        68, // on Operator, goto 67
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        69, // on Value, goto 68
        70, // on Variable, goto 69
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 115
        0, // on (BitOperator BitValue), error
        52, // on (BitOperator BitValue)+, goto 51
        0, // on @L, error
        0, // on @R, error
        53, // on Application, goto 52
        54, // on ApplicationBits, goto 53
        55, // on Atom, goto 54
        0, // on BitApplication, error
        0, // on BitApplications, error
        56, // on BitOperator, goto 55
        0, // on BitOperator?, error
        57, // on BitValue, goto 56
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        60, // on Fact<FactDataApply>, goto 59
        140, // on Fact<FactDataFunc>, goto 139
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        64, // on FactDataApply, goto 63
        141, // on FactDataFunc, goto 140
        0, // on FactDataOr, error
        67, // on Identifier, goto 66
        0, // on Item, error
        0, // on Item+, error
        68, // on Operator, goto 67
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        69, // on Value, goto 68
        70, // on Variable, goto 69
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 116
        0, // on (BitOperator BitValue), error
        52, // on (BitOperator BitValue)+, goto 51
        0, // on @L, error
        0, // on @R, error
        53, // on Application, goto 52
        54, // on ApplicationBits, goto 53
        55, // on Atom, goto 54
        0, // on BitApplication, error
        0, // on BitApplications, error
        56, // on BitOperator, goto 55
        0, // on BitOperator?, error
        57, // on BitValue, goto 56
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        60, // on Fact<FactDataApply>, goto 59
        142, // on Fact<FactDataFunc>, goto 141
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        64, // on FactDataApply, goto 63
        141, // on FactDataFunc, goto 140
        0, // on FactDataOr, error
        67, // on Identifier, goto 66
        0, // on Item, error
        0, // on Item+, error
        68, // on Operator, goto 67
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        69, // on Value, goto 68
        70, // on Variable, goto 69
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 117
        0, // on (BitOperator BitValue), error
        80, // on (BitOperator BitValue)+, goto 79
        0, // on @L, error
        0, // on @R, error
        81, // on Application, goto 80
        82, // on ApplicationBits, goto 81
        83, // on Atom, goto 82
        84, // on BitApplication, goto 83
        143, // on BitApplications, goto 142
        86, // on BitOperator, goto 85
        0, // on BitOperator?, error
        87, // on BitValue, goto 86
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        88, // on Identifier, goto 87
        0, // on Item, error
        0, // on Item+, error
        89, // on Operator, goto 88
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        90, // on Value, goto 89
        91, // on Variable, goto 90
        92, // on Vec1<BitApplication>, goto 91
        0, // on __Program, error
        // State 118
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 119
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        145, // on Identifier, goto 144
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        146, // on Variable, goto 145
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 120
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        145, // on Identifier, goto 144
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        148, // on Variable, goto 147
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 121
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        83, // on Atom, goto 82
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        149, // on BitValue, goto 148
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        125, // on Identifier, goto 124
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        90, // on Value, goto 89
        91, // on Variable, goto 90
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 122
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 123
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 124
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 125
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 126
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        150, // on BitOperator, goto 149
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        89, // on Operator, goto 88
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 127
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        83, // on Atom, goto 82
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        124, // on BitValue, goto 123
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        125, // on Identifier, goto 124
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        90, // on Value, goto 89
        91, // on Variable, goto 90
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 128
        0, // on (BitOperator BitValue), error
        80, // on (BitOperator BitValue)+, goto 79
        0, // on @L, error
        0, // on @R, error
        81, // on Application, goto 80
        82, // on ApplicationBits, goto 81
        83, // on Atom, goto 82
        84, // on BitApplication, goto 83
        151, // on BitApplications, goto 150
        86, // on BitOperator, goto 85
        0, // on BitOperator?, error
        87, // on BitValue, goto 86
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        88, // on Identifier, goto 87
        0, // on Item, error
        0, // on Item+, error
        89, // on Operator, goto 88
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        90, // on Value, goto 89
        91, // on Variable, goto 90
        92, // on Vec1<BitApplication>, goto 91
        0, // on __Program, error
        // State 129
        0, // on (BitOperator BitValue), error
        80, // on (BitOperator BitValue)+, goto 79
        0, // on @L, error
        0, // on @R, error
        81, // on Application, goto 80
        82, // on ApplicationBits, goto 81
        83, // on Atom, goto 82
        152, // on BitApplication, goto 151
        0, // on BitApplications, error
        86, // on BitOperator, goto 85
        0, // on BitOperator?, error
        87, // on BitValue, goto 86
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        88, // on Identifier, goto 87
        0, // on Item, error
        0, // on Item+, error
        89, // on Operator, goto 88
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        90, // on Value, goto 89
        91, // on Variable, goto 90
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 130
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 131
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 132
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        38, // on Atom, goto 37
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        132, // on BitValue, goto 131
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        102, // on Identifier, goto 101
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        43, // on Value, goto 42
        44, // on Variable, goto 43
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 133
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 134
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 135
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 136
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        55, // on Atom, goto 54
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        136, // on BitValue, goto 135
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        110, // on Identifier, goto 109
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        69, // on Value, goto 68
        70, // on Variable, goto 69
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 137
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 138
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 139
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 140
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 141
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 142
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 143
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 144
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 145
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 146
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 147
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 148
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 149
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        83, // on Atom, goto 82
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        149, // on BitValue, goto 148
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        125, // on Identifier, goto 124
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        90, // on Value, goto 89
        91, // on Variable, goto 90
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 150
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 151
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 152
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 153
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 154
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 155
        0, // on (BitOperator BitValue), error
        159, // on (BitOperator BitValue)+, goto 158
        0, // on @L, error
        0, // on @R, error
        160, // on Application, goto 159
        161, // on ApplicationBits, goto 160
        162, // on Atom, goto 161
        0, // on BitApplication, error
        0, // on BitApplications, error
        163, // on BitOperator, goto 162
        0, // on BitOperator?, error
        164, // on BitValue, goto 163
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        165, // on Fact<FactDataApply>, goto 164
        166, // on Fact<FactDataFunc>, goto 165
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        167, // on FactDataApply, goto 166
        168, // on FactDataFunc, goto 167
        0, // on FactDataOr, error
        169, // on Identifier, goto 168
        0, // on Item, error
        0, // on Item+, error
        170, // on Operator, goto 169
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        171, // on Value, goto 170
        172, // on Variable, goto 171
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 156
        0, // on (BitOperator BitValue), error
        159, // on (BitOperator BitValue)+, goto 158
        0, // on @L, error
        0, // on @R, error
        160, // on Application, goto 159
        161, // on ApplicationBits, goto 160
        162, // on Atom, goto 161
        0, // on BitApplication, error
        0, // on BitApplications, error
        163, // on BitOperator, goto 162
        0, // on BitOperator?, error
        164, // on BitValue, goto 163
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        165, // on Fact<FactDataApply>, goto 164
        181, // on Fact<FactDataFunc>, goto 180
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        167, // on FactDataApply, goto 166
        168, // on FactDataFunc, goto 167
        0, // on FactDataOr, error
        169, // on Identifier, goto 168
        0, // on Item, error
        0, // on Item+, error
        170, // on Operator, goto 169
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        171, // on Value, goto 170
        172, // on Variable, goto 171
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 157
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 158
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        182, // on BitOperator, goto 181
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        170, // on Operator, goto 169
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 159
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 160
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 161
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 162
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        162, // on Atom, goto 161
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        183, // on BitValue, goto 182
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        184, // on Identifier, goto 183
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        171, // on Value, goto 170
        172, // on Variable, goto 171
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 163
        0, // on (BitOperator BitValue), error
        186, // on (BitOperator BitValue)+, goto 185
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        187, // on BitOperator, goto 186
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        30, // on Operator, goto 29
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 164
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 165
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 166
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 167
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 168
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 169
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 170
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 171
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 172
        0, // on (BitOperator BitValue), error
        35, // on (BitOperator BitValue)+, goto 34
        0, // on @L, error
        0, // on @R, error
        191, // on Application, goto 190
        37, // on ApplicationBits, goto 36
        38, // on Atom, goto 37
        0, // on BitApplication, error
        0, // on BitApplications, error
        39, // on BitOperator, goto 38
        0, // on BitOperator?, error
        40, // on BitValue, goto 39
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        41, // on Identifier, goto 40
        0, // on Item, error
        0, // on Item+, error
        42, // on Operator, goto 41
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        43, // on Value, goto 42
        44, // on Variable, goto 43
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 173
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 174
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 175
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 176
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 177
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 178
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 179
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 180
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 181
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        162, // on Atom, goto 161
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        195, // on BitValue, goto 194
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        184, // on Identifier, goto 183
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        171, // on Value, goto 170
        172, // on Variable, goto 171
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 182
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 183
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 184
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 185
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        196, // on BitOperator, goto 195
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        170, // on Operator, goto 169
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 186
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        162, // on Atom, goto 161
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        183, // on BitValue, goto 182
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        184, // on Identifier, goto 183
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        171, // on Value, goto 170
        172, // on Variable, goto 171
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 187
        0, // on (BitOperator BitValue), error
        159, // on (BitOperator BitValue)+, goto 158
        0, // on @L, error
        0, // on @R, error
        160, // on Application, goto 159
        161, // on ApplicationBits, goto 160
        162, // on Atom, goto 161
        0, // on BitApplication, error
        0, // on BitApplications, error
        163, // on BitOperator, goto 162
        0, // on BitOperator?, error
        164, // on BitValue, goto 163
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        165, // on Fact<FactDataApply>, goto 164
        197, // on Fact<FactDataFunc>, goto 196
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        167, // on FactDataApply, goto 166
        168, // on FactDataFunc, goto 167
        0, // on FactDataOr, error
        169, // on Identifier, goto 168
        0, // on Item, error
        0, // on Item+, error
        170, // on Operator, goto 169
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        171, // on Value, goto 170
        172, // on Variable, goto 171
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 188
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 189
        0, // on (BitOperator BitValue), error
        80, // on (BitOperator BitValue)+, goto 79
        0, // on @L, error
        0, // on @R, error
        81, // on Application, goto 80
        82, // on ApplicationBits, goto 81
        83, // on Atom, goto 82
        84, // on BitApplication, goto 83
        198, // on BitApplications, goto 197
        86, // on BitOperator, goto 85
        0, // on BitOperator?, error
        87, // on BitValue, goto 86
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        88, // on Identifier, goto 87
        0, // on Item, error
        0, // on Item+, error
        89, // on Operator, goto 88
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        90, // on Value, goto 89
        91, // on Variable, goto 90
        92, // on Vec1<BitApplication>, goto 91
        0, // on __Program, error
        // State 190
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 191
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        145, // on Identifier, goto 144
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        200, // on Variable, goto 199
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 192
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        145, // on Identifier, goto 144
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        201, // on Variable, goto 200
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 193
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 194
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 195
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        162, // on Atom, goto 161
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        195, // on BitValue, goto 194
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        184, // on Identifier, goto 183
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        171, // on Value, goto 170
        172, // on Variable, goto 171
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 196
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 197
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 198
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 199
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 200
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 201
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 202
        0, // on (BitOperator BitValue), error
        159, // on (BitOperator BitValue)+, goto 158
        0, // on @L, error
        0, // on @R, error
        160, // on Application, goto 159
        161, // on ApplicationBits, goto 160
        162, // on Atom, goto 161
        0, // on BitApplication, error
        0, // on BitApplications, error
        163, // on BitOperator, goto 162
        0, // on BitOperator?, error
        164, // on BitValue, goto 163
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        165, // on Fact<FactDataApply>, goto 164
        205, // on Fact<FactDataFunc>, goto 204
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        167, // on FactDataApply, goto 166
        168, // on FactDataFunc, goto 167
        0, // on FactDataOr, error
        169, // on Identifier, goto 168
        0, // on Item, error
        0, // on Item+, error
        170, // on Operator, goto 169
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        171, // on Value, goto 170
        172, // on Variable, goto 171
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 203
        0, // on (BitOperator BitValue), error
        159, // on (BitOperator BitValue)+, goto 158
        0, // on @L, error
        0, // on @R, error
        160, // on Application, goto 159
        161, // on ApplicationBits, goto 160
        162, // on Atom, goto 161
        0, // on BitApplication, error
        0, // on BitApplications, error
        163, // on BitOperator, goto 162
        0, // on BitOperator?, error
        164, // on BitValue, goto 163
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        165, // on Fact<FactDataApply>, goto 164
        206, // on Fact<FactDataFunc>, goto 205
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        167, // on FactDataApply, goto 166
        168, // on FactDataFunc, goto 167
        0, // on FactDataOr, error
        169, // on Identifier, goto 168
        0, // on Item, error
        0, // on Item+, error
        170, // on Operator, goto 169
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        171, // on Value, goto 170
        172, // on Variable, goto 171
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 204
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 205
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 206
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
        // State 207
        0, // on (BitOperator BitValue), error
        0, // on (BitOperator BitValue)+, error
        0, // on @L, error
        0, // on @R, error
        0, // on Application, error
        0, // on ApplicationBits, error
        0, // on Atom, error
        0, // on BitApplication, error
        0, // on BitApplications, error
        0, // on BitOperator, error
        0, // on BitOperator?, error
        0, // on BitValue, error
        0, // on BitValue?, error
        0, // on Fact<FactData>, error
        0, // on Fact<FactDataAnd>, error
        0, // on Fact<FactDataApply>, error
        0, // on Fact<FactDataFunc>, error
        0, // on Fact<FactDataOr>, error
        0, // on FactData, error
        0, // on FactDataAnd, error
        0, // on FactDataApply, error
        0, // on FactDataFunc, error
        0, // on FactDataOr, error
        0, // on Identifier, error
        0, // on Item, error
        0, // on Item+, error
        0, // on Operator, error
        0, // on OperatorValue, error
        0, // on Program, error
        0, // on Rule, error
        0, // on Value, error
        0, // on Variable, error
        0, // on Vec1<BitApplication>, error
        0, // on __Program, error
    ];
    pub fn parse_Program<
        'input,
    >(
        input: &'input str,
    ) -> Result<Program, __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        '__shift: loop {
            let __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            let __integer = match __lookahead {
                (_, (0, _), _) if true => 0,
                (_, (1, _), _) if true => 1,
                (_, (2, _), _) if true => 2,
                (_, (3, _), _) if true => 3,
                (_, (4, _), _) if true => 4,
                (_, (5, _), _) if true => 5,
                (_, (6, _), _) if true => 6,
                (_, (7, _), _) if true => 7,
                (_, (8, _), _) if true => 8,
                (_, (9, _), _) if true => 9,
                (_, (10, _), _) if true => 10,
                (_, (11, _), _) if true => 11,
                (_, (12, _), _) if true => 12,
                (_, (13, _), _) if true => 13,
                (_, (14, _), _) if true => 14,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 15 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_29_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_2c_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2d_3e_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_2e_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_3a_2d_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_3b_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_3d_3e_22(__tok0),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22___22(__tok0),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22exists_22(__tok0),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22forall_22(__tok0),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Termr_23_22_5c_27_5b_5e_5c_27_5d_2b_5c_27_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Termr_23_22_5b_2d_7c_21_40_23_24_25_5e_26_2a_3d_2b_2f_3a_3f_7e_3c_3e_5d_2b_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Termr_23_22_5bA_2dZa_2dz_5d_5bA_2dZa_2dz0_2d9___5d_2a_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Termr_23_22_5bA_2dZa_2dz_5d_5bA_2dZa_2dz0_2d9___5d_2a_3a_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: vec![],
                });
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Program,__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (BitOperator BitValue) = BitOperator, BitValue => ActionFn(41);
                let __sym1 = __pop_NtBitValue(__symbols);
                let __sym0 = __pop_NtBitOperator(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action41::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28BitOperator_20BitValue_29(__nt), __end));
                0
            }
            2 => {
                // (BitOperator BitValue)+ = BitOperator, BitValue => ActionFn(53);
                let __sym1 = __pop_NtBitValue(__symbols);
                let __sym0 = __pop_NtBitOperator(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action53::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28BitOperator_20BitValue_29_2b(__nt), __end));
                1
            }
            3 => {
                // (BitOperator BitValue)+ = (BitOperator BitValue)+, BitOperator, BitValue => ActionFn(54);
                let __sym2 = __pop_NtBitValue(__symbols);
                let __sym1 = __pop_NtBitOperator(__symbols);
                let __sym0 = __pop_Nt_28BitOperator_20BitValue_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action54::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28BitOperator_20BitValue_29_2b(__nt), __end));
                1
            }
            4 => {
                // @L =  => ActionFn(50);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action50::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_40L(__nt), __end));
                2
            }
            5 => {
                // @R =  => ActionFn(48);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action48::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_40R(__nt), __end));
                3
            }
            6 => {
                // Application = ApplicationBits => ActionFn(66);
                let __sym0 = __pop_NtApplicationBits(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action66::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtApplication(__nt), __end));
                4
            }
            7 => {
                // ApplicationBits = BitValue => ActionFn(16);
                let __sym0 = __pop_NtBitValue(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtApplicationBits(__nt), __end));
                5
            }
            8 => {
                // ApplicationBits = BitOperator => ActionFn(17);
                let __sym0 = __pop_NtBitOperator(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtApplicationBits(__nt), __end));
                5
            }
            9 => {
                // ApplicationBits = BitValue, (BitOperator BitValue)+, BitOperator => ActionFn(79);
                let __sym2 = __pop_NtBitOperator(__symbols);
                let __sym1 = __pop_Nt_28BitOperator_20BitValue_29_2b(__symbols);
                let __sym0 = __pop_NtBitValue(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action79::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtApplicationBits(__nt), __end));
                5
            }
            10 => {
                // ApplicationBits = (BitOperator BitValue)+, BitOperator => ActionFn(80);
                let __sym1 = __pop_NtBitOperator(__symbols);
                let __sym0 = __pop_Nt_28BitOperator_20BitValue_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action80::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtApplicationBits(__nt), __end));
                5
            }
            11 => {
                // ApplicationBits = BitValue, (BitOperator BitValue)+ => ActionFn(81);
                let __sym1 = __pop_Nt_28BitOperator_20BitValue_29_2b(__symbols);
                let __sym0 = __pop_NtBitValue(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action81::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtApplicationBits(__nt), __end));
                5
            }
            12 => {
                // ApplicationBits = (BitOperator BitValue)+ => ActionFn(82);
                let __sym0 = __pop_Nt_28BitOperator_20BitValue_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action82::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtApplicationBits(__nt), __end));
                5
            }
            13 => {
                // ApplicationBits = Identifier, "(", BitApplications, ")" => ActionFn(67);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtBitApplications(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_NtIdentifier(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action67::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtApplicationBits(__nt), __end));
                5
            }
            14 => {
                // Atom = r#"\'[^\']+\'"# => ActionFn(31);
                let __sym0 = __pop_Termr_23_22_5c_27_5b_5e_5c_27_5d_2b_5c_27_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                6
            }
            15 => {
                // BitApplication = Application => ActionFn(68);
                let __sym0 = __pop_NtApplication(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action68::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBitApplication(__nt), __end));
                7
            }
            16 => {
                // BitApplications = Vec1<BitApplication> => ActionFn(20);
                let __sym0 = __pop_NtVec1_3cBitApplication_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBitApplications(__nt), __end));
                8
            }
            17 => {
                // BitOperator = Operator => ActionFn(69);
                let __sym0 = __pop_NtOperator(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action69::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBitOperator(__nt), __end));
                9
            }
            18 => {
                // BitOperator? = BitOperator => ActionFn(37);
                let __sym0 = __pop_NtBitOperator(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action37::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBitOperator_3f(__nt), __end));
                10
            }
            19 => {
                // BitOperator? =  => ActionFn(38);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action38::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtBitOperator_3f(__nt), __end));
                10
            }
            20 => {
                // BitValue = Value => ActionFn(70);
                let __sym0 = __pop_NtValue(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action70::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBitValue(__nt), __end));
                11
            }
            21 => {
                // BitValue? = BitValue => ActionFn(42);
                let __sym0 = __pop_NtBitValue(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action42::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBitValue_3f(__nt), __end));
                12
            }
            22 => {
                // BitValue? =  => ActionFn(43);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action43::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtBitValue_3f(__nt), __end));
                12
            }
            23 => {
                // Fact<FactData> = FactData => ActionFn(71);
                let __sym0 = __pop_NtFactData(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action71::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFact_3cFactData_3e(__nt), __end));
                13
            }
            24 => {
                // Fact<FactDataAnd> = FactDataAnd => ActionFn(72);
                let __sym0 = __pop_NtFactDataAnd(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action72::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFact_3cFactDataAnd_3e(__nt), __end));
                14
            }
            25 => {
                // Fact<FactDataApply> = FactDataApply => ActionFn(73);
                let __sym0 = __pop_NtFactDataApply(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action73::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFact_3cFactDataApply_3e(__nt), __end));
                15
            }
            26 => {
                // Fact<FactDataFunc> = FactDataFunc => ActionFn(74);
                let __sym0 = __pop_NtFactDataFunc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action74::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFact_3cFactDataFunc_3e(__nt), __end));
                16
            }
            27 => {
                // Fact<FactDataOr> = FactDataOr => ActionFn(75);
                let __sym0 = __pop_NtFactDataOr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action75::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFact_3cFactDataOr_3e(__nt), __end));
                17
            }
            28 => {
                // FactData = FactDataAnd => ActionFn(5);
                let __sym0 = __pop_NtFactDataAnd(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFactData(__nt), __end));
                18
            }
            29 => {
                // FactDataAnd = FactDataOr => ActionFn(6);
                let __sym0 = __pop_NtFactDataOr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFactDataAnd(__nt), __end));
                19
            }
            30 => {
                // FactDataAnd = Fact<FactDataAnd>, ";", Fact<FactDataOr> => ActionFn(7);
                let __sym2 = __pop_NtFact_3cFactDataOr_3e(__symbols);
                let __sym1 = __pop_Term_22_3b_22(__symbols);
                let __sym0 = __pop_NtFact_3cFactDataAnd_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action7::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactDataAnd(__nt), __end));
                19
            }
            31 => {
                // FactDataApply = Application => ActionFn(14);
                let __sym0 = __pop_NtApplication(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFactDataApply(__nt), __end));
                20
            }
            32 => {
                // FactDataFunc = FactDataApply => ActionFn(10);
                let __sym0 = __pop_NtFactDataApply(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFactDataFunc(__nt), __end));
                21
            }
            33 => {
                // FactDataFunc = Fact<FactDataApply>, "=>", Fact<FactDataFunc> => ActionFn(11);
                let __sym2 = __pop_NtFact_3cFactDataFunc_3e(__symbols);
                let __sym1 = __pop_Term_22_3d_3e_22(__symbols);
                let __sym0 = __pop_NtFact_3cFactDataApply_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action11::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactDataFunc(__nt), __end));
                21
            }
            34 => {
                // FactDataFunc = "exists", "(", Variable, "->", Fact<FactDataFunc>, ")" => ActionFn(12);
                let __sym5 = __pop_Term_22_29_22(__symbols);
                let __sym4 = __pop_NtFact_3cFactDataFunc_3e(__symbols);
                let __sym3 = __pop_Term_22_2d_3e_22(__symbols);
                let __sym2 = __pop_NtVariable(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_Term_22exists_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action12::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtFactDataFunc(__nt), __end));
                21
            }
            35 => {
                // FactDataFunc = "forall", "(", Variable, "->", Fact<FactDataFunc>, ")" => ActionFn(13);
                let __sym5 = __pop_Term_22_29_22(__symbols);
                let __sym4 = __pop_NtFact_3cFactDataFunc_3e(__symbols);
                let __sym3 = __pop_Term_22_2d_3e_22(__symbols);
                let __sym2 = __pop_NtVariable(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_Term_22forall_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action13::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtFactDataFunc(__nt), __end));
                21
            }
            36 => {
                // FactDataOr = FactDataFunc => ActionFn(8);
                let __sym0 = __pop_NtFactDataFunc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFactDataOr(__nt), __end));
                22
            }
            37 => {
                // FactDataOr = Fact<FactDataOr>, ",", Fact<FactDataFunc> => ActionFn(9);
                let __sym2 = __pop_NtFact_3cFactDataFunc_3e(__symbols);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtFact_3cFactDataOr_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action9::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactDataOr(__nt), __end));
                22
            }
            38 => {
                // Identifier = r#"[A-Za-z][A-Za-z0-9_]*"# => ActionFn(33);
                let __sym0 = __pop_Termr_23_22_5bA_2dZa_2dz_5d_5bA_2dZa_2dz0_2d9___5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdentifier(__nt), __end));
                23
            }
            39 => {
                // Item = Application, "." => ActionFn(2);
                let __sym1 = __pop_Term_22_2e_22(__symbols);
                let __sym0 = __pop_NtApplication(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action2::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtItem(__nt), __end));
                24
            }
            40 => {
                // Item = Rule => ActionFn(3);
                let __sym0 = __pop_NtRule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtItem(__nt), __end));
                24
            }
            41 => {
                // Item+ = Item => ActionFn(51);
                let __sym0 = __pop_NtItem(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action51::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtItem_2b(__nt), __end));
                25
            }
            42 => {
                // Item+ = Item+, Item => ActionFn(52);
                let __sym1 = __pop_NtItem(__symbols);
                let __sym0 = __pop_NtItem_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action52::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtItem_2b(__nt), __end));
                25
            }
            43 => {
                // Operator = r#"[A-Za-z][A-Za-z0-9_]*:"# => ActionFn(24);
                let __sym0 = __pop_Termr_23_22_5bA_2dZa_2dz_5d_5bA_2dZa_2dz0_2d9___5d_2a_3a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOperator(__nt), __end));
                26
            }
            44 => {
                // Operator = r#"[-|!@#$%^&*=+/:?~<>]+"# => ActionFn(25);
                let __sym0 = __pop_Termr_23_22_5b_2d_7c_21_40_23_24_25_5e_26_2a_3d_2b_2f_3a_3f_7e_3c_3e_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOperator(__nt), __end));
                26
            }
            45 => {
                // OperatorValue = Operator, Value => ActionFn(22);
                let __sym1 = __pop_NtValue(__symbols);
                let __sym0 = __pop_NtOperator(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action22::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtOperatorValue(__nt), __end));
                27
            }
            46 => {
                // Program = Item+ => ActionFn(1);
                let __sym0 = __pop_NtItem_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                28
            }
            47 => {
                // Rule = Application, ":-", Fact<FactData>, "." => ActionFn(76);
                let __sym3 = __pop_Term_22_2e_22(__symbols);
                let __sym2 = __pop_NtFact_3cFactData_3e(__symbols);
                let __sym1 = __pop_Term_22_3a_2d_22(__symbols);
                let __sym0 = __pop_NtApplication(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action76::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtRule(__nt), __end));
                29
            }
            48 => {
                // Value = Atom => ActionFn(27);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtValue(__nt), __end));
                30
            }
            49 => {
                // Value = Variable => ActionFn(28);
                let __sym0 = __pop_NtVariable(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtValue(__nt), __end));
                30
            }
            50 => {
                // Value = "(", Application, ")" => ActionFn(29);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtApplication(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action29::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtValue(__nt), __end));
                30
            }
            51 => {
                // Value = "_" => ActionFn(30);
                let __sym0 = __pop_Term_22___22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtValue(__nt), __end));
                30
            }
            52 => {
                // Variable = Identifier => ActionFn(32);
                let __sym0 = __pop_NtIdentifier(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtVariable(__nt), __end));
                31
            }
            53 => {
                // Vec1<BitApplication> = BitApplication => ActionFn(34);
                let __sym0 = __pop_NtBitApplication(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtVec1_3cBitApplication_3e(__nt), __end));
                32
            }
            54 => {
                // Vec1<BitApplication> = Vec1<BitApplication>, "," => ActionFn(35);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtVec1_3cBitApplication_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action35::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtVec1_3cBitApplication_3e(__nt), __end));
                32
            }
            55 => {
                // Vec1<BitApplication> = Vec1<BitApplication>, ",", BitApplication => ActionFn(36);
                let __sym2 = __pop_NtBitApplication(__symbols);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtVec1_3cBitApplication_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action36::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtVec1_3cBitApplication_3e(__nt), __end));
                32
            }
            56 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_NtProgram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 34 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22___22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22___22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22exists_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22exists_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22forall_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22forall_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5c_27_5b_5e_5c_27_5d_2b_5c_27_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5c_27_5b_5e_5c_27_5d_2b_5c_27_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b_2d_7c_21_40_23_24_25_5e_26_2a_3d_2b_2f_3a_3f_7e_3c_3e_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b_2d_7c_21_40_23_24_25_5e_26_2a_3d_2b_2f_3a_3f_7e_3c_3e_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5bA_2dZa_2dz_5d_5bA_2dZa_2dz0_2d9___5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5bA_2dZa_2dz_5d_5bA_2dZa_2dz0_2d9___5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5bA_2dZa_2dz_5d_5bA_2dZa_2dz0_2d9___5d_2a_3a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5bA_2dZa_2dz_5d_5bA_2dZa_2dz0_2d9___5d_2a_3a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28BitOperator_20BitValue_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (Bit, Bit), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28BitOperator_20BitValue_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28BitOperator_20BitValue_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(Bit, Bit)>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28BitOperator_20BitValue_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_40L<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_40L(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_40R<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_40R(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtApplication<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Application, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtApplication(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtApplicationBits<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Bit>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtApplicationBits(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAtom<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Atom, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAtom(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBitApplication<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Bit, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBitApplication(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBitApplications<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Bit>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBitApplications(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBitOperator<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Bit, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBitOperator(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBitOperator_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Bit>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBitOperator_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBitValue<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Bit, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBitValue(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBitValue_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Bit>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBitValue_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFact_3cFactData_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Fact, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFact_3cFactData_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFact_3cFactDataAnd_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Fact, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFact_3cFactDataAnd_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFact_3cFactDataApply_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Fact, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFact_3cFactDataApply_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFact_3cFactDataFunc_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Fact, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFact_3cFactDataFunc_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFact_3cFactDataOr_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Fact, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFact_3cFactDataOr_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFactData<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<FactData>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFactData(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFactDataAnd<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<FactData>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFactDataAnd(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFactDataApply<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<FactData>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFactDataApply(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFactDataFunc<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<FactData>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFactDataFunc(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFactDataOr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<FactData>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFactDataOr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtIdentifier<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, InternedString, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtIdentifier(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtItem<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Item, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtItem(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtItem_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Item>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtItem_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOperator<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Operator, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOperator(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOperatorValue<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (Operator, Value), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOperatorValue(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtProgram<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Program, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtProgram(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtRule<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Rule, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtRule(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtValue<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Value, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtValue(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtVariable<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Variable, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtVariable(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtVec1_3cBitApplication_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Bit>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtVec1_3cBitApplication_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Program<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Program, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Program(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Program::parse_Program;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        33 => /* '!' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        35 ... 38 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        39 => /* '\'' */ {
                            __current_state = 2;
                            continue;
                        }
                        40 => /* '(' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        41 => /* ')' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        42 ... 43 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        44 => /* ',' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        45 => /* '-' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        46 => /* '.' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        47 => /* '/' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        59 => /* ';' */ {
                            __current_match = Some((6, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        60 => /* '<' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        61 => /* '=' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        62 ... 64 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        94 => /* '^' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((8, __index + 1));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 13;
                            continue;
                        }
                        102 => /* 'f' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 14;
                            continue;
                        }
                        103 ... 122 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        124 => /* '|' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        126 => /* '~' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        33 => /* '!' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        35 ... 38 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        42 ... 43 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        45 => /* '-' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        47 => /* '/' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        60 ... 64 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        94 => /* '^' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        124 => /* '|' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        126 => /* '~' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        0 ... 38 => {
                            __current_state = 17;
                            continue;
                        }
                        40 ... 1114111 => {
                            __current_state = 17;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        33 => /* '!' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        35 ... 38 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        42 ... 43 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        45 => /* '-' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        47 => /* '/' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        60 ... 61 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        62 => /* '>' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 18;
                            continue;
                        }
                        63 ... 64 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        94 => /* '^' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        124 => /* '|' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        126 => /* '~' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        33 => /* '!' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        35 ... 38 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        42 ... 43 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        45 => /* '-' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 19;
                            continue;
                        }
                        47 => /* '/' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        60 ... 64 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        94 => /* '^' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        124 => /* '|' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        126 => /* '~' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                10 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        33 => /* '!' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        35 ... 38 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        42 ... 43 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        45 => /* '-' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        47 => /* '/' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        60 ... 61 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        62 => /* '>' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 20;
                            continue;
                        }
                        63 ... 64 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        94 => /* '^' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        124 => /* '|' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        126 => /* '~' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                11 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 21;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                12 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                13 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 21;
                            continue;
                        }
                        97 ... 119 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        120 => /* 'x' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 23;
                            continue;
                        }
                        121 ... 122 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                14 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 21;
                            continue;
                        }
                        97 ... 110 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        111 => /* 'o' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 24;
                            continue;
                        }
                        112 ... 122 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                15 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                16 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        33 => /* '!' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        35 ... 38 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        42 ... 43 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        45 => /* '-' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        47 => /* '/' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        60 ... 64 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        94 => /* '^' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        124 => /* '|' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        126 => /* '~' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                17 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        0 ... 38 => {
                            __current_state = 25;
                            continue;
                        }
                        39 => /* '\'' */ {
                            __current_match = Some((11, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        40 ... 1114111 => {
                            __current_state = 25;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                18 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        33 => /* '!' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        35 ... 38 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        42 ... 43 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        45 => /* '-' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        47 => /* '/' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        60 ... 64 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        94 => /* '^' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        124 => /* '|' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        126 => /* '~' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                19 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        33 => /* '!' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        35 ... 38 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        42 ... 43 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        45 => /* '-' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        47 => /* '/' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        60 ... 64 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        94 => /* '^' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        124 => /* '|' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        126 => /* '~' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                20 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        33 => /* '!' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        35 ... 38 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        42 ... 43 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        45 => /* '-' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        47 => /* '/' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        60 ... 64 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        94 => /* '^' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        124 => /* '|' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        126 => /* '~' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                21 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 21;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                22 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                23 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 21;
                            continue;
                        }
                        97 ... 104 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        105 => /* 'i' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 27;
                            continue;
                        }
                        106 ... 122 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                24 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 21;
                            continue;
                        }
                        97 ... 113 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 28;
                            continue;
                        }
                        115 ... 122 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                25 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        0 ... 38 => {
                            __current_state = 25;
                            continue;
                        }
                        39 => /* '\'' */ {
                            __current_match = Some((11, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        40 ... 1114111 => {
                            __current_state = 25;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                26 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                27 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 21;
                            continue;
                        }
                        97 ... 114 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        115 => /* 's' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 29;
                            continue;
                        }
                        116 ... 122 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                28 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 21;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        98 ... 122 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                29 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 21;
                            continue;
                        }
                        97 ... 115 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 31;
                            continue;
                        }
                        117 ... 122 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                30 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 21;
                            continue;
                        }
                        97 ... 107 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        108 => /* 'l' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 32;
                            continue;
                        }
                        109 ... 122 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                31 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 21;
                            continue;
                        }
                        97 ... 114 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        115 => /* 's' */ {
                            __current_match = Some((9, __index + 1));
                            __current_state = 33;
                            continue;
                        }
                        116 ... 122 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                32 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 21;
                            continue;
                        }
                        97 ... 107 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        108 => /* 'l' */ {
                            __current_match = Some((10, __index + 1));
                            __current_state = 34;
                            continue;
                        }
                        109 ... 122 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                33 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 21;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                34 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 21;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__lalrpop_util::ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

#[allow(unused_variables)]
pub fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Program, usize),
) -> Program
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ::std::vec::Vec<Item>, usize),
) -> Program
{
    Program { items: __0 }
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, a, _): (usize, Application, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Item
{
    Item::Fact(a)
}

#[allow(unused_variables)]
pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Rule, usize),
) -> Item
{
    Item::Rule(__0)
}

#[allow(unused_variables)]
pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, s0, _): (usize, usize, usize),
    (_, a, _): (usize, Application, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, f, _): (usize, Fact, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, s1, _): (usize, usize, usize),
) -> Rule
{
    Rule {
        span: Span::new(s0, s1),
        consequence: a,
        condition: f
    }
}

#[allow(unused_variables)]
pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<FactData>, usize),
) -> Box<FactData>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action6<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<FactData>, usize),
) -> Box<FactData>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action7<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Fact, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, Fact, usize),
) -> Box<FactData>
{
    Box::new(FactData::And(l, r))
}

#[allow(unused_variables)]
pub fn __action8<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<FactData>, usize),
) -> Box<FactData>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action9<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Fact, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, Fact, usize),
) -> Box<FactData>
{
    Box::new(FactData::Or(l, r))
}

#[allow(unused_variables)]
pub fn __action10<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<FactData>, usize),
) -> Box<FactData>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action11<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Fact, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, Fact, usize),
) -> Box<FactData>
{
    Box::new(FactData::Implication(l, r))
}

#[allow(unused_variables)]
pub fn __action12<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, Variable, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, b, _): (usize, Fact, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Box<FactData>
{
    Box::new(FactData::Exists(v, b))
}

#[allow(unused_variables)]
pub fn __action13<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, Variable, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, b, _): (usize, Fact, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Box<FactData>
{
    Box::new(FactData::ForAll(v, b))
}

#[allow(unused_variables)]
pub fn __action14<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Application, usize),
) -> Box<FactData>
{
    Box::new(FactData::Apply(__0))
}

#[allow(unused_variables)]
pub fn __action15<
    'input,
>(
    input: &'input str,
    (_, s0, _): (usize, usize, usize),
    (_, bits, _): (usize, Vec<Bit>, usize),
    (_, s1, _): (usize, usize, usize),
) -> Application
{
    {
        Application {
            span: Span::new(s0, s1),
            bits: bits
        }
    }
}

#[allow(unused_variables)]
pub fn __action16<
    'input,
>(
    input: &'input str,
    (_, head, _): (usize, Bit, usize),
) -> Vec<Bit>
{
    vec![head]
}

#[allow(unused_variables)]
pub fn __action17<
    'input,
>(
    input: &'input str,
    (_, head, _): (usize, Bit, usize),
) -> Vec<Bit>
{
    vec![head]
}

#[allow(unused_variables)]
pub fn __action18<
    'input,
>(
    input: &'input str,
    (_, head, _): (usize, ::std::option::Option<Bit>, usize),
    (_, body, _): (usize, ::std::vec::Vec<(Bit, Bit)>, usize),
    (_, tail, _): (usize, ::std::option::Option<Bit>, usize),
) -> Vec<Bit>
{
    head.into_iter()
            .chain(body.into_iter().flat_map(|(o, v)| once(o).chain(once(v))))
            .chain(tail)
            .collect()
}

#[allow(unused_variables)]
pub fn __action19<
    'input,
>(
    input: &'input str,
    (_, s0, _): (usize, usize, usize),
    (_, id, _): (usize, InternedString, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, appls, _): (usize, Vec<Bit>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, s1, _): (usize, usize, usize),
) -> Vec<Bit>
{
    {
        let oper_bit = Bit {
            span: Span::new(s0, s1),
            kind: BitKind::Operator(Operator::Parens(id))
        };
        Some(oper_bit).into_iter().chain(appls).collect()
    }
}

#[allow(unused_variables)]
pub fn __action20<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<Bit>, usize),
) -> Vec<Bit>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action21<
    'input,
>(
    input: &'input str,
    (_, s0, _): (usize, usize, usize),
    (_, a, _): (usize, Application, usize),
    (_, s1, _): (usize, usize, usize),
) -> Bit
{
    {
        Bit {
            span: Span::new(s0, s1),
            kind: BitKind::Value(Value::Application(a)),
        }
    }
}

#[allow(unused_variables)]
pub fn __action22<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Operator, usize),
    (_, __1, _): (usize, Value, usize),
) -> (Operator, Value)
{
    (__0, __1)
}

#[allow(unused_variables)]
pub fn __action23<
    'input,
>(
    input: &'input str,
    (_, s0, _): (usize, usize, usize),
    (_, head, _): (usize, Operator, usize),
    (_, s1, _): (usize, usize, usize),
) -> Bit
{
    Bit { span: Span::new(s0, s1), kind: BitKind::Operator(head) }
}

#[allow(unused_variables)]
pub fn __action24<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Operator
{
    Operator::Colon(intern(__0))
}

#[allow(unused_variables)]
pub fn __action25<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Operator
{
    Operator::Symbols(intern(__0))
}

#[allow(unused_variables)]
pub fn __action26<
    'input,
>(
    input: &'input str,
    (_, s0, _): (usize, usize, usize),
    (_, head, _): (usize, Value, usize),
    (_, s1, _): (usize, usize, usize),
) -> Bit
{
    Bit { span: Span::new(s0, s1), kind: BitKind::Value(head) }
}

#[allow(unused_variables)]
pub fn __action27<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Atom, usize),
) -> Value
{
    Value::Atom(__0)
}

#[allow(unused_variables)]
pub fn __action28<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Variable, usize),
) -> Value
{
    Value::Variable(__0)
}

#[allow(unused_variables)]
pub fn __action29<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Application, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Value
{
    Value::Application(__0)
}

#[allow(unused_variables)]
pub fn __action30<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Value
{
    Value::Wildcard
}

#[allow(unused_variables)]
pub fn __action31<
    'input,
>(
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> Atom
{
    Atom { id: intern(&s[1..s.len() - 1]) }
}

#[allow(unused_variables)]
pub fn __action32<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, InternedString, usize),
) -> Variable
{
    Variable { id: __0 }
}

#[allow(unused_variables)]
pub fn __action33<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> InternedString
{
    intern(__0)
}

#[allow(unused_variables)]
pub fn __action34<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, Bit, usize),
) -> Vec<Bit>
{
    vec![v]
}

#[allow(unused_variables)]
pub fn __action35<
    'input,
>(
    input: &'input str,
    (_, vs, _): (usize, Vec<Bit>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Vec<Bit>
{
    vs
}

#[allow(unused_variables)]
pub fn __action36<
    'input,
>(
    input: &'input str,
    (_, vs, _): (usize, Vec<Bit>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, Bit, usize),
) -> Vec<Bit>
{
    {
        let mut vs = vs;
        vs.push(v);
        vs
    }
}

#[allow(unused_variables)]
pub fn __action37<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Bit, usize),
) -> ::std::option::Option<Bit>
{
    Some(__0)
}

#[allow(unused_variables)]
pub fn __action38<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Bit>
{
    None
}

#[allow(unused_variables)]
pub fn __action39<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, (Bit, Bit), usize),
) -> ::std::vec::Vec<(Bit, Bit)>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action40<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<(Bit, Bit)>, usize),
    (_, e, _): (usize, (Bit, Bit), usize),
) -> ::std::vec::Vec<(Bit, Bit)>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action41<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Bit, usize),
    (_, __1, _): (usize, Bit, usize),
) -> (Bit, Bit)
{
    (__0, __1)
}

#[allow(unused_variables)]
pub fn __action42<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Bit, usize),
) -> ::std::option::Option<Bit>
{
    Some(__0)
}

#[allow(unused_variables)]
pub fn __action43<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Bit>
{
    None
}

#[allow(unused_variables)]
pub fn __action44<
    'input,
>(
    input: &'input str,
    (_, s0, _): (usize, usize, usize),
    (_, d, _): (usize, Box<FactData>, usize),
    (_, s1, _): (usize, usize, usize),
) -> Fact
{
    Fact {
            span: Span::new(s0, s1),
            data: d
        }
}

#[allow(unused_variables)]
pub fn __action45<
    'input,
>(
    input: &'input str,
    (_, s0, _): (usize, usize, usize),
    (_, d, _): (usize, Box<FactData>, usize),
    (_, s1, _): (usize, usize, usize),
) -> Fact
{
    Fact {
            span: Span::new(s0, s1),
            data: d
        }
}

#[allow(unused_variables)]
pub fn __action46<
    'input,
>(
    input: &'input str,
    (_, s0, _): (usize, usize, usize),
    (_, d, _): (usize, Box<FactData>, usize),
    (_, s1, _): (usize, usize, usize),
) -> Fact
{
    Fact {
            span: Span::new(s0, s1),
            data: d
        }
}

#[allow(unused_variables)]
pub fn __action47<
    'input,
>(
    input: &'input str,
    (_, s0, _): (usize, usize, usize),
    (_, d, _): (usize, Box<FactData>, usize),
    (_, s1, _): (usize, usize, usize),
) -> Fact
{
    Fact {
            span: Span::new(s0, s1),
            data: d
        }
}

#[allow(unused_variables)]
pub fn __action48<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookbehind.clone()
}

#[allow(unused_variables)]
pub fn __action49<
    'input,
>(
    input: &'input str,
    (_, s0, _): (usize, usize, usize),
    (_, d, _): (usize, Box<FactData>, usize),
    (_, s1, _): (usize, usize, usize),
) -> Fact
{
    Fact {
            span: Span::new(s0, s1),
            data: d
        }
}

#[allow(unused_variables)]
pub fn __action50<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookahead.clone()
}

#[allow(unused_variables)]
pub fn __action51<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Item, usize),
) -> ::std::vec::Vec<Item>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action52<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Item>, usize),
    (_, e, _): (usize, Item, usize),
) -> ::std::vec::Vec<Item>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action53<
    'input,
>(
    input: &'input str,
    __0: (usize, Bit, usize),
    __1: (usize, Bit, usize),
) -> ::std::vec::Vec<(Bit, Bit)>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action41(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action39(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action54<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<(Bit, Bit)>, usize),
    __1: (usize, Bit, usize),
    __2: (usize, Bit, usize),
) -> ::std::vec::Vec<(Bit, Bit)>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action41(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action40(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action55<
    'input,
>(
    input: &'input str,
    __0: (usize, Vec<Bit>, usize),
    __1: (usize, usize, usize),
) -> Application
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action50(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action15(
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
pub fn __action56<
    'input,
>(
    input: &'input str,
    __0: (usize, InternedString, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Vec<Bit>, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, usize, usize),
) -> Vec<Bit>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action50(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action19(
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
pub fn __action57<
    'input,
>(
    input: &'input str,
    __0: (usize, Application, usize),
    __1: (usize, usize, usize),
) -> Bit
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action50(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action21(
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
pub fn __action58<
    'input,
>(
    input: &'input str,
    __0: (usize, Operator, usize),
    __1: (usize, usize, usize),
) -> Bit
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action50(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action23(
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
pub fn __action59<
    'input,
>(
    input: &'input str,
    __0: (usize, Value, usize),
    __1: (usize, usize, usize),
) -> Bit
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action50(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action26(
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
pub fn __action60<
    'input,
>(
    input: &'input str,
    __0: (usize, Box<FactData>, usize),
    __1: (usize, usize, usize),
) -> Fact
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action50(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action49(
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
pub fn __action61<
    'input,
>(
    input: &'input str,
    __0: (usize, Box<FactData>, usize),
    __1: (usize, usize, usize),
) -> Fact
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action50(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action47(
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
pub fn __action62<
    'input,
>(
    input: &'input str,
    __0: (usize, Box<FactData>, usize),
    __1: (usize, usize, usize),
) -> Fact
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action50(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action44(
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
pub fn __action63<
    'input,
>(
    input: &'input str,
    __0: (usize, Box<FactData>, usize),
    __1: (usize, usize, usize),
) -> Fact
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action50(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action45(
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
pub fn __action64<
    'input,
>(
    input: &'input str,
    __0: (usize, Box<FactData>, usize),
    __1: (usize, usize, usize),
) -> Fact
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action50(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action46(
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
pub fn __action65<
    'input,
>(
    input: &'input str,
    __0: (usize, Application, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Fact, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, usize, usize),
) -> Rule
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action50(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
pub fn __action66<
    'input,
>(
    input: &'input str,
    __0: (usize, Vec<Bit>, usize),
) -> Application
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action48(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action55(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action67<
    'input,
>(
    input: &'input str,
    __0: (usize, InternedString, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Vec<Bit>, usize),
    __3: (usize, &'input str, usize),
) -> Vec<Bit>
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action48(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action56(
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action68<
    'input,
>(
    input: &'input str,
    __0: (usize, Application, usize),
) -> Bit
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action48(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action57(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action69<
    'input,
>(
    input: &'input str,
    __0: (usize, Operator, usize),
) -> Bit
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action48(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action58(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action70<
    'input,
>(
    input: &'input str,
    __0: (usize, Value, usize),
) -> Bit
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action48(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action59(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action71<
    'input,
>(
    input: &'input str,
    __0: (usize, Box<FactData>, usize),
) -> Fact
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action48(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action60(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action72<
    'input,
>(
    input: &'input str,
    __0: (usize, Box<FactData>, usize),
) -> Fact
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action48(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action61(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action73<
    'input,
>(
    input: &'input str,
    __0: (usize, Box<FactData>, usize),
) -> Fact
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action48(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action62(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action74<
    'input,
>(
    input: &'input str,
    __0: (usize, Box<FactData>, usize),
) -> Fact
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action48(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action63(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action75<
    'input,
>(
    input: &'input str,
    __0: (usize, Box<FactData>, usize),
) -> Fact
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action48(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action64(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action76<
    'input,
>(
    input: &'input str,
    __0: (usize, Application, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Fact, usize),
    __3: (usize, &'input str, usize),
) -> Rule
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action48(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action65(
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action77<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::option::Option<Bit>, usize),
    __1: (usize, ::std::vec::Vec<(Bit, Bit)>, usize),
    __2: (usize, Bit, usize),
) -> Vec<Bit>
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action37(
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action18(
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action78<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::option::Option<Bit>, usize),
    __1: (usize, ::std::vec::Vec<(Bit, Bit)>, usize),
) -> Vec<Bit>
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action38(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action18(
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action79<
    'input,
>(
    input: &'input str,
    __0: (usize, Bit, usize),
    __1: (usize, ::std::vec::Vec<(Bit, Bit)>, usize),
    __2: (usize, Bit, usize),
) -> Vec<Bit>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action42(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action77(
        input,
        __temp0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
pub fn __action80<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<(Bit, Bit)>, usize),
    __1: (usize, Bit, usize),
) -> Vec<Bit>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action77(
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
pub fn __action81<
    'input,
>(
    input: &'input str,
    __0: (usize, Bit, usize),
    __1: (usize, ::std::vec::Vec<(Bit, Bit)>, usize),
) -> Vec<Bit>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action42(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action78(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
pub fn __action82<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<(Bit, Bit)>, usize),
) -> Vec<Bit>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action43(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action78(
        input,
        __temp0,
        __0,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}