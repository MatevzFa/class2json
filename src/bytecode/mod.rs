macro_rules! instruction {
    ($instr:expr, [$(eval $a:expr),*]) => {{
        let mut vector: Vec<i32> = Vec::new();
        $(vector.push($x as u8);)*
        (instr, vector.sum(), vector)
    }};
}

pub fn to_string(bytecode: u8) -> (&'static str, i32, Vec<u32>) {
    match bytecode {
        0x19 => ("aload", 1, vec![1]),
        0x3a => ("astore", 1, vec![1]),
        0x10 => ("bipush", 1, vec![1]),
        0x18 => ("dload", 1, vec![1]),
        0x39 => ("dstore", 1, vec![1]),
        0x17 => ("fload", 1, vec![1]),
        0x38 => ("fstore", 1, vec![1]),
        0x15 => ("iload", 1, vec![1]),
        0x36 => ("istore", 1, vec![1]),
        0x12 => ("ldc", 1, vec![1]),
        0x16 => ("lload", 1, vec![1]),
        0x37 => ("lstore", 1, vec![1]),
        0xbc => ("newarray", 1, vec![1]),
        0xa9 => ("ret", 1, vec![1]),
        0xbd => ("anewarray", 2, vec![2]),
        0xc0 => ("checkcast", 2, vec![2]),
        0xb4 => ("getfield", 2, vec![2]),
        0xb2 => ("getstatic", 2, vec![2]),
        0xa7 => ("goto", 2, vec![2]),
        0xa5 => ("if_acmpeq", 2, vec![2]),
        0xa6 => ("if_acmpne", 2, vec![2]),
        0x9f => ("if_icmpeq", 2, vec![2]),
        0xa2 => ("if_icmpge", 2, vec![2]),
        0xa3 => ("if_icmpgt", 2, vec![2]),
        0xa4 => ("if_icmple", 2, vec![2]),
        0xa1 => ("if_icmplt", 2, vec![2]),
        0xa0 => ("if_icmpne", 2, vec![2]),
        0x99 => ("ifeq", 2, vec![2]),
        0x9c => ("ifge", 2, vec![2]),
        0x9d => ("ifgt", 2, vec![2]),
        0x9e => ("ifle", 2, vec![2]),
        0x9b => ("iflt", 2, vec![2]),
        0x9a => ("ifne", 2, vec![2]),
        0xc7 => ("ifnonnull", 2, vec![2]),
        0xc6 => ("ifnull", 2, vec![2]),
        0x84 => ("iinc", 2, vec![1, 1]),
        0xc1 => ("instanceof", 2, vec![2]),
        0xb7 => ("invokespecial", 2, vec![2]),
        0xb8 => ("invokestatic", 2, vec![2]),
        0xb6 => ("invokevirtual", 2, vec![2]),
        0xa8 => ("jsr", 2, vec![2]),
        0x13 => ("ldc_w", 2, vec![2]),
        0x14 => ("ldc2_w", 2, vec![2]),
        0xbb => ("new", 2, vec![2]),
        0xb5 => ("putfield", 2, vec![2]),
        0xb3 => ("putstatic", 2, vec![2]),
        0x11 => ("sipush", 2, vec![2]),
        0xc5 => ("multianewarray", 3, vec![2, 1]),
        0xc8 => ("goto_w", 4, vec![4]),
        0xba => ("invokedynamic", 4, vec![2, 1, 1]),
        0xb9 => ("invokeinterface", 4, vec![2, 1, 1]),
        0xc9 => ("jsr_w", 4, vec![4]),
        0xaa => ("tableswitch", -1, vec![]),
        0xc4 => ("wide", -1, vec![]),
        0xab => ("lookupswitch", -1, vec![]),
        0x32 => ("aaload", 0, vec![]),
        0x53 => ("aastore", 0, vec![]),
        0x01 => ("const_null", 0, vec![]),
        0x2a => ("aload_0", 0, vec![]),
        0x2b => ("aload_1", 0, vec![]),
        0x2c => ("aload_2", 0, vec![]),
        0x2d => ("aload_3", 0, vec![]),
        0xb0 => ("areturn", 0, vec![]),
        0xbe => ("arraylength", 0, vec![]),
        0x4b => ("astore_0", 0, vec![]),
        0x4c => ("astore_1", 0, vec![]),
        0x4d => ("astore_2", 0, vec![]),
        0x4e => ("astore_3", 0, vec![]),
        0xbf => ("athrow", 0, vec![]),
        0x33 => ("baload", 0, vec![]),
        0x54 => ("bastore", 0, vec![]),
        0xca => ("breakpoint", 0, vec![]),
        0x34 => ("caload", 0, vec![]),
        0x55 => ("castore", 0, vec![]),
        0x90 => ("d2f", 0, vec![]),
        0x8e => ("d2i", 0, vec![]),
        0x8f => ("d2l", 0, vec![]),
        0x63 => ("dadd", 0, vec![]),
        0x31 => ("daload", 0, vec![]),
        0x52 => ("dastore", 0, vec![]),
        0x98 => ("dcmpg", 0, vec![]),
        0x97 => ("dcmpl", 0, vec![]),
        0x0e => ("dconst_0", 0, vec![]),
        0x0f => ("dconst_1", 0, vec![]),
        0x6f => ("ddiv", 0, vec![]),
        0x26 => ("dload_0", 0, vec![]),
        0x27 => ("dload_1", 0, vec![]),
        0x28 => ("dload_2", 0, vec![]),
        0x29 => ("dload_3", 0, vec![]),
        0x6b => ("dmul", 0, vec![]),
        0x77 => ("dneg", 0, vec![]),
        0x73 => ("drem", 0, vec![]),
        0xaf => ("dreturn", 0, vec![]),
        0x47 => ("dstore_0", 0, vec![]),
        0x48 => ("dstore_1", 0, vec![]),
        0x49 => ("dstore_2", 0, vec![]),
        0x4a => ("dstore_3", 0, vec![]),
        0x67 => ("dsub", 0, vec![]),
        0x59 => ("dup", 0, vec![]),
        0x5a => ("dup_x1", 0, vec![]),
        0x5b => ("dup_x2", 0, vec![]),
        0x5c => ("dup2", 0, vec![]),
        0x5d => ("dup2_x1", 0, vec![]),
        0x5e => ("dup2_x2", 0, vec![]),
        0x8d => ("f2d", 0, vec![]),
        0x8b => ("f2i", 0, vec![]),
        0x8c => ("f2l", 0, vec![]),
        0x62 => ("fadd", 0, vec![]),
        0x30 => ("faload", 0, vec![]),
        0x51 => ("fastore", 0, vec![]),
        0x96 => ("fcmpg", 0, vec![]),
        0x95 => ("fcmpl", 0, vec![]),
        0x0b => ("fconst_0", 0, vec![]),
        0x0c => ("fconst_1", 0, vec![]),
        0x0d => ("fconst_2", 0, vec![]),
        0x6e => ("fdiv", 0, vec![]),
        0x22 => ("fload_0", 0, vec![]),
        0x23 => ("fload_1", 0, vec![]),
        0x24 => ("fload_2", 0, vec![]),
        0x25 => ("fload_3", 0, vec![]),
        0x6a => ("fmul", 0, vec![]),
        0x76 => ("fneg", 0, vec![]),
        0x72 => ("frem", 0, vec![]),
        0xae => ("freturn", 0, vec![]),
        0x43 => ("fstore_0", 0, vec![]),
        0x44 => ("fstore_1", 0, vec![]),
        0x45 => ("fstore_2", 0, vec![]),
        0x46 => ("fstore_3", 0, vec![]),
        0x66 => ("fsub", 0, vec![]),
        0x91 => ("i2b", 0, vec![]),
        0x92 => ("i2c", 0, vec![]),
        0x87 => ("i2d", 0, vec![]),
        0x86 => ("i2f", 0, vec![]),
        0x85 => ("i2l", 0, vec![]),
        0x93 => ("i2s", 0, vec![]),
        0x60 => ("iadd", 0, vec![]),
        0x2e => ("iaload", 0, vec![]),
        0x7e => ("iand", 0, vec![]),
        0x4f => ("iastore", 0, vec![]),
        0x02 => ("const_m1", 0, vec![]),
        0x03 => ("const_0", 0, vec![]),
        0x04 => ("const_1", 0, vec![]),
        0x05 => ("const_2", 0, vec![]),
        0x06 => ("const_3", 0, vec![]),
        0x07 => ("const_4", 0, vec![]),
        0x08 => ("const_5", 0, vec![]),
        0x6c => ("idiv", 0, vec![]),
        0x1a => ("iload_0", 0, vec![]),
        0x1b => ("iload_1", 0, vec![]),
        0x1c => ("iload_2", 0, vec![]),
        0x1d => ("iload_3", 0, vec![]),
        0xfe => ("impdep1", 0, vec![]),
        0xff => ("impdep2", 0, vec![]),
        0x68 => ("imul", 0, vec![]),
        0x74 => ("ineg", 0, vec![]),
        0x80 => ("ior", 0, vec![]),
        0x70 => ("irem", 0, vec![]),
        0xac => ("ireturn", 0, vec![]),
        0x78 => ("ishl", 0, vec![]),
        0x7a => ("ishr", 0, vec![]),
        0x3b => ("istore_0", 0, vec![]),
        0x3c => ("istore_1", 0, vec![]),
        0x3d => ("istore_2", 0, vec![]),
        0x3e => ("istore_3", 0, vec![]),
        0x64 => ("isub", 0, vec![]),
        0x7c => ("iushr", 0, vec![]),
        0x82 => ("ixor", 0, vec![]),
        0x8a => ("l2d", 0, vec![]),
        0x89 => ("l2f", 0, vec![]),
        0x88 => ("l2i", 0, vec![]),
        0x61 => ("ladd", 0, vec![]),
        0x2f => ("laload", 0, vec![]),
        0x7f => ("land", 0, vec![]),
        0x50 => ("lastore", 0, vec![]),
        0x94 => ("lcmp", 0, vec![]),
        0x09 => ("const_0", 0, vec![]),
        0x0a => ("lconst_1", 0, vec![]),
        0x6d => ("ldiv", 0, vec![]),
        0x1e => ("lload_0", 0, vec![]),
        0x1f => ("lload_1", 0, vec![]),
        0x20 => ("lload_2", 0, vec![]),
        0x21 => ("lload_3", 0, vec![]),
        0x69 => ("lmul", 0, vec![]),
        0x75 => ("lneg", 0, vec![]),
        0x81 => ("lor", 0, vec![]),
        0x71 => ("lrem", 0, vec![]),
        0xad => ("lreturn", 0, vec![]),
        0x79 => ("lshl", 0, vec![]),
        0x7b => ("lshr", 0, vec![]),
        0x3f => ("lstore_0", 0, vec![]),
        0x40 => ("lstore_1", 0, vec![]),
        0x41 => ("lstore_2", 0, vec![]),
        0x42 => ("lstore_3", 0, vec![]),
        0x65 => ("lsub", 0, vec![]),
        0x7d => ("lushr", 0, vec![]),
        0x83 => ("lxor", 0, vec![]),
        0xc2 => ("monitorenter", 0, vec![]),
        0xc3 => ("monitorexit", 0, vec![]),
        0x00 => ("op", 0, vec![]),
        0x57 => ("pop", 0, vec![]),
        0x58 => ("pop2", 0, vec![]),
        0xb1 => ("return", 0, vec![]),
        0x35 => ("saload", 0, vec![]),
        0x56 => ("sastore", 0, vec![]),
        0x5f => ("swap", 0, vec![]),

        _ => panic!("Unknown bytecode {:x}", bytecode),
    }
}
