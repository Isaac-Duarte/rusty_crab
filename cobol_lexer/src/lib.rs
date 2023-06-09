#[derive(Debug)]
/// Parsed token
pub struct Token {
    pub kind: TokenKind,
    pub column: u32,
}

impl Token {
    fn new(kind: TokenKind, column: u32) -> Token {
        Token { kind, column }
    }
}

pub enum CobolKeyword {
    Accept,
    Access,
    Add,
    Address,
    Advancing,
    After,
    All,
    Alphabet,
    AlphabeticLower,
    AlphabeticUpper,
    Alphabetic,
    AlphanumericEdited,
    Alphanumeric,
    Also,
    Alter,
    Alternate,
    And,
    Any,
    Apply,
    Are,
    Area,
    Areas,
    Ascending,
    Assign,
    At,
    Author,
    Basis,
    Before,
    Beginning,
    Binary,
    Blank,
    Block,
    Bottom,
    By,
    Call,
    Cancel,
    Cbl,
    Cd,
    Cf,
    Ch,
    Character,
    Characters,
    ClassId,
    Class,
    ClockUnits,
    Close,
    Cobol,
    CodeSet,
    Code,
    Collating,
    Column,
    ComReg,
    Comma,
    Common,
    Communication,
    Comp1,
    Comp2,
    Comp3,
    Comp4,
    Comp5,
    Comp,
    Computational1,
    Computational2,
    Computational3,
    Computational4,
    Computational5,
    Computational,
    Compute,
    Configuration,
    Contains,
    Content,
    Continue,
    Control,
    Controls,
    Converting,
    Copy,
    Corr,
    Corresponding,
    Count,
    Currency,
    Data,
    DateCompiled,
    DateWritten,
    DayOfWeek,
    Day,
    Dbcs,
    De,
    DebugContents,
    DebugItem,
    DebugLine,
    DebugName,
    DebugSub1,
    DebugSub2,
    DebugSub3,
    Debugging,
    DecimalPoint,
    Declaratives,
    Delete,
    Delimited,
    Delimiter,
    Depending,
    Descending,
    Destination,
    Detail,
    Display1,
    Display,
    Divide,
    Division,
    Down,
    Duplicates,
    Dynamic,
    Egcs,
    Egi,
    Eject,
    Else,
    Emi,
    Enable,
    EndAdd,
    EndCall,
    EndCompute,
    EndDelete,
    EndDivide,
    EndEvaluate,
    EndIf,
    EndInvoke,
    EndMultiply,
    EndOfPage,
    EndPerform,
    EndRead,
    EndReceive,
    EndReturn,
    EndRewrite,
    EndSearch,
    EndStart,
    EndString,
    EndSubtract,
    EndUnstring,
    EndWrite,
    End,
    Ending,
    Enter,
    Entry,
    Environment,
    Eop,
    Equal,
    Error,
    Esi,
    Evaluate,
    Every,
    Exception,
    Exit,
    Extend,
    External,
    False,
    Fd,
    FileControl,
    File,
    Filler,
    Final,
    First,
    Footing,
    For,
    From,
    Function,
    Generate,
    Giving,
    Global,
    Go,
    Goback,
    Greater,
    Group,
    Heading,
    HighValue,
    HighValues,
    IOControl,
    IO,
    Id,
    Identification,
    If,
    In,
    Index,
    Indexed,
    Indicate,
    Inherits,
    Initial,
    Initialize,
    Initiate,
    InputOutput,
    Input,
    Insert,
    Inspect,
    Installation,
    Into,
    Invalid,
    Invoke,
    Is,
    Just,
    Justified,
    Kanji,
    Key,
    Label,
    Last,
    Leading,
    Left,
    Length,
    Less,
    Limit,
    Limits,
    LinageCounter,
    Linage,
    LineCounter,
    Line,
    Lines,
    Linkage,
    LocalStorage,
    Lock,
    LowValue,
    LowValues,
    Memory,
    Merge,
    Message,
    Metaclass,
    MethodId,
    Method,
    Mode,
    Modules,
    MoreLabels,
    Move,
    Multiple,
    Multiply,
    NativeBinary,
    Native,
    Negative,
    Next,
    No,
    Not,
    Null,
    Nulls,
    Number,
    NumericEdited,
    Numeric,
    ObjectComputer,
    Object,
    Occurs,
    Of,
    Off,
    Omitted,
    On,
    Open,
    Optional,
    Or,
    Order,
    Organization,
    Other,
    Output,
    Overflow,
    Override,
    PackedDecimal,
    Padding,
    PageCounter,
    Page,
    Password,
    Perform,
    Pf,
    Ph,
    Pic,
    Picture,
    Plus,
    Pointer,
    Position,
    Positive,
    Printing,
    ProcedurePointer,
    Procedure,
    Procedures,
    Proceed,
    Processing,
    ProgramId,
    Program,
    Purge,
    Queue,
    Quote,
    Quotes,
    Random,
    Rd,
    Read,
    Ready,
    Receive,
    Record,
    Recording,
    Records,
    Recursive,
    Redefines,
    Reel,
    Reference,
    References,
    Relative,
    Release,
    Reload,
    Remainder,
    Removal,
    Renames,
    Replace,
    Replacing,
    Report,
    Reporting,
    Reports,
    Repository,
    Rerun,
    Reserve,
    Reset,
    ReturnCode,
    Return,
    Returning,
    Reversed,
    Rewind,
    Rewrite,
    Rf,
    Rh,
    Right,
    Rounded,
    Run,
    Same,
    Sd,
    Search,
    Section,
    Security,
    SegmentLimit,
    Segment,
    Select,
    SELF,
    Send,
    Sentence,
    Separate,
    Sequence,
    Sequential,
    Service,
    Set,
    ShiftIn,
    ShiftOut,
    Sign,
    Size,
    Skip1,
    Skip2,
    Skip3,
    SortControl,
    SortCoreSize,
    SortFileSize,
    SortMerge,
    SortMessage,
    SortModeSize,
    SortReturn,
    Sort,
    SourceComputer,
    Source,
    Space,
    Spaces,
    SpecialNames,
    Standard1,
    Standard2,
    Standard,
    Start,
    Status,
    Stop,
    String,
    SubQueue1,
    SubQueue2,
    SubQueue3,
    Subtract,
    Sum,
    Super,
    Suppress,
    Symbolic,
    Sync,
    Synchronized,
    Table,
    Tally,
    Tallying,
    Tape,
    Terminal,
    Terminate,
    Test,
    Text,
    Than,
    Then,
    Through,
    Thru,
    Time,
    Times,
    Title,
    To,
    Top,
    Trace,
    Trailing,
    True,
    Type,
    Unit,
    Unstring,
    Until,
    Up,
    Upon,
    Usage,
    Use,
    Using,
    Value,
    Values,
    Varying,
    WhenCompiled,
    When,
    With,
    Words,
    WorkingStorage,
    WriteOnly,
    Write,
    Zero,
    Zeroes,
    Zeros,
}

#[derive(Debug)]
pub enum NonTerminal {
    Plus,
    Minus,
    Astrik,
    Equals,
    Pipe,
    Comma,
    Colon,
    SemiColon,
    LessThan,
    GreaterThan,
    Ampersand,
    Underscore,
}

#[derive(Debug)]
pub enum TokenKind {
    Comment(String),
    Keyword(String),
    Number(f64),
    Literal(String),
    Identifier(String),
    Symbol(NonTerminal),
    Period,
}
