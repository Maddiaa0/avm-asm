use phf::phf_map;

/// All  opcodes
/// Keep updated with TS, cpp, and docs protocol specs!
#[allow(clippy::upper_case_acronyms, dead_code)]
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Opcode {
    // Compute
    ADD,
    SUB,
    MUL,
    DIV,
    FDIV,
    EQ,
    LT,
    LTE,
    AND,
    OR,
    XOR,
    NOT,
    SHL,
    SHR,
    CAST,
    // Execution environment
    ADDRESS,
    STORAGEADDRESS,
    SENDER,
    FEEPERL2GAS,
    FEEPERDAGAS,
    TRANSACTIONFEE,
    CONTRACTCALLDEPTH,
    CHAINID,
    VERSION,
    BLOCKNUMBER,
    TIMESTAMP,
    COINBASE,
    BLOCKL2GASLIMIT,
    BLOCKDAGASLIMIT,
    CALLDATACOPY,
    // Gas
    L2GASLEFT,
    DAGASLEFT,
    // Control flow
    JUMP,
    JUMPI,
    INTERNALCALL,
    INTERNALRETURN,
    // Memory
    SET,
    MOV,
    CMOV,
    // World state
    SLOAD,
    SSTORE,
    NOTEHASHEXISTS,
    EMITNOTEHASH,
    NULLIFIEREXISTS,
    EMITNULLIFIER,
    L1TOL2MSGEXISTS,
    HEADERMEMBER,
    GETCONTRACTINSTANCE,
    EMITUNENCRYPTEDLOG,
    SENDL2TOL1MSG,
    // External calls
    CALL,
    STATICCALL,
    DELEGATECALL,
    RETURN,
    REVERT,
    // Misc
    DEBUGLOG,
    // Gadgets
    KECCAK,
    POSEIDON2,
    SHA256,   // temp - may be removed, but alot of contracts rely on it
    PEDERSEN, // temp - may be removed, but alot of contracts rely on it
    ECADD,
    MSM,
    // Conversions
    TORADIXLE,
}

impl Opcode {
    pub fn name(&self) -> &'static str {
        match self {
            // Compute
            // Compute - Arithmetic
            Opcode::ADD => "ADD",
            Opcode::SUB => "SUB",
            Opcode::MUL => "MUL",
            Opcode::DIV => "DIV",
            Opcode::FDIV => "FDIV",
            // Compute - Comparators
            Opcode::EQ => "EQ",
            Opcode::LT => "LT",
            Opcode::LTE => "LTE",
            // Compute - Bitwise
            Opcode::AND => "AND",
            Opcode::OR => "OR",
            Opcode::XOR => "XOR",
            Opcode::NOT => "NOT",
            Opcode::SHL => "SHL",
            Opcode::SHR => "SHR",
            // Compute - Type Conversions
            Opcode::CAST => "CAST",

            // Execution Environment
            Opcode::ADDRESS => "ADDRESS",
            Opcode::STORAGEADDRESS => "STORAGEADDRESS",
            Opcode::SENDER => "SENDER",
            Opcode::FEEPERL2GAS => "FEEPERL2GAS",
            Opcode::FEEPERDAGAS => "FEEPERDAGAS",
            Opcode::TRANSACTIONFEE => "TRANSACTIONFEE",
            Opcode::CONTRACTCALLDEPTH => "CONTRACTCALLDEPTH",
            // Execution Environment - Globals
            Opcode::CHAINID => "CHAINID",
            Opcode::VERSION => "VERSION",
            Opcode::BLOCKNUMBER => "BLOCKNUMBER",
            Opcode::TIMESTAMP => "TIMESTAMP",
            Opcode::COINBASE => "COINBASE",
            Opcode::BLOCKL2GASLIMIT => "BLOCKL2GASLIMIT",
            Opcode::BLOCKDAGASLIMIT => "BLOCKDAGASLIMIT",
            // Execution Environment - Calldata
            Opcode::CALLDATACOPY => "CALLDATACOPY",

            // Machine State
            // Machine State - Gas
            Opcode::L2GASLEFT => "L2GASLEFT",
            Opcode::DAGASLEFT => "DAGASLEFT",
            // Machine State - Internal Control Flow
            Opcode::JUMP => "JUMP",
            Opcode::JUMPI => "JUMPI",
            Opcode::INTERNALCALL => "INTERNALCALL",
            Opcode::INTERNALRETURN => "INTERNALRETURN",
            // Machine State - Memory
            Opcode::SET => "SET",
            Opcode::MOV => "MOV",
            Opcode::CMOV => "CMOV",

            // World State
            Opcode::SLOAD => "SLOAD",                     // Public Storage
            Opcode::SSTORE => "SSTORE",                   // Public Storage
            Opcode::NOTEHASHEXISTS => "NOTEHASHEXISTS",   // Notes & Nullifiers
            Opcode::EMITNOTEHASH => "EMITNOTEHASH",       // Notes & Nullifiers
            Opcode::NULLIFIEREXISTS => "NULLIFIEREXISTS", // Notes & Nullifiers
            Opcode::EMITNULLIFIER => "EMITNULLIFIER",     // Notes & Nullifiers
            Opcode::L1TOL2MSGEXISTS => "L1TOL2MSGEXISTS", // Messages
            Opcode::HEADERMEMBER => "HEADERMEMBER",       // Archive tree & Headers

            // Accrued Substate
            Opcode::EMITUNENCRYPTEDLOG => "EMITUNENCRYPTEDLOG",
            Opcode::SENDL2TOL1MSG => "SENDL2TOL1MSG",
            Opcode::GETCONTRACTINSTANCE => "GETCONTRACTINSTANCE",

            // Control Flow - Contract Calls
            Opcode::CALL => "CALL",
            Opcode::STATICCALL => "STATICCALL",
            Opcode::DELEGATECALL => "DELEGATECALL",
            Opcode::RETURN => "RETURN",
            Opcode::REVERT => "REVERT",

            // Misc
            Opcode::DEBUGLOG => "DEBUGLOG",

            // Gadgets
            Opcode::KECCAK => "KECCAK",
            Opcode::POSEIDON2 => "POSEIDON2",
            Opcode::SHA256 => "SHA256 ",
            Opcode::PEDERSEN => "PEDERSEN",
            Opcode::ECADD => "ECADD",
            Opcode::MSM => "MSM",
            // Conversions
            Opcode::TORADIXLE => "TORADIXLE",
        }
    }

    pub fn has_tag(&self) -> bool {
        match self {
            Opcode::CAST | Opcode::SET => true,
            _ => false,
        }
    }
}

pub static OPCODE_MAP: phf::Map<&'static str, Opcode> = phf_map! {
    "add" => Opcode::ADD,
    "sub" => Opcode::SUB,
    "mul" => Opcode::MUL,
    "div" => Opcode::DIV,
    "fdiV" => Opcode::FDIV,
    "eq" => Opcode::EQ,
    "lt" => Opcode::LT,
    "lte" => Opcode::LTE,
    "and" => Opcode::AND,
    "or" => Opcode::OR,
    "xor" => Opcode::XOR,
    "not" => Opcode::NOT,
    "shl" => Opcode::SHL,
    "shr" => Opcode::SHR,
    "cast" => Opcode::CAST,
    "address" => Opcode::ADDRESS,
    "storageaddress" => Opcode::STORAGEADDRESS,
    "sender" => Opcode::SENDER,
    "feeperl2gas" => Opcode::FEEPERL2GAS,
    "feeperdagas" => Opcode::FEEPERDAGAS,
    "transactionfee" => Opcode::TRANSACTIONFEE,
    "contractcalldepth" => Opcode::CONTRACTCALLDEPTH,
    "chainid" => Opcode::CHAINID,
    "version" => Opcode::VERSION,
    "blocknumber" => Opcode::BLOCKNUMBER,
    "timestamp" => Opcode::TIMESTAMP,
    "coinbase" => Opcode::COINBASE,
    "blockl2gaslimit" => Opcode::BLOCKL2GASLIMIT,
    "blockdagaslimit" => Opcode::BLOCKDAGASLIMIT,
    "calldatacopy" => Opcode::CALLDATACOPY,
    "l2gasleft" => Opcode::L2GASLEFT,
    "dagasleft" => Opcode::DAGASLEFT,
    "jump" => Opcode::JUMP,
    "jumpi" => Opcode::JUMPI,
    "internalcall" => Opcode::INTERNALCALL,
    "internalreturn" => Opcode::INTERNALRETURN,
    "set" => Opcode::SET,
    "mov" => Opcode::MOV,
    "cmov" => Opcode::CMOV,
    "sload" => Opcode::SLOAD,
    "sstore" => Opcode::SSTORE,
    "notehashexists" => Opcode::NOTEHASHEXISTS,
    "emitnotehash" => Opcode::EMITNOTEHASH,
    "nullifierexists" => Opcode::NULLIFIEREXISTS,
    "emitnullifier" => Opcode::EMITNULLIFIER,
    "l1tol2msgexists" => Opcode::L1TOL2MSGEXISTS,
    "headermember" => Opcode::HEADERMEMBER,
    "getcontractinstance" => Opcode::GETCONTRACTINSTANCE,
    "emitunencryptedlog" => Opcode::EMITUNENCRYPTEDLOG,
    "sendl2tol1msg" => Opcode::SENDL2TOL1MSG,
    "call" => Opcode::CALL,
    "staticcall" => Opcode::STATICCALL,
    "delegatecall" => Opcode::DELEGATECALL,
    "return" => Opcode::RETURN,
    "revert" => Opcode::REVERT,
    "debuglog" => Opcode::DEBUGLOG,
    "keccak" => Opcode::KECCAK,
    "poseidon2" => Opcode::POSEIDON2,
    "sha256" => Opcode::SHA256,
    "pedersen" => Opcode::PEDERSEN,
    "ecadd" => Opcode::ECADD,
    "msm" => Opcode::MSM,
    "toradixle" => Opcode::TORADIXLE,
};
