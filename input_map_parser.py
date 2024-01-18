def parse_input_maps():
    f = open("input_maps/flip_down.txt", "r")
    for x in f:
        line = x.strip()
        pline = line.split("//")
        if len(pline) > 1:
            func = pline[0].strip().strip("const").strip().strip("unsigned").strip()
            comment = pline[1].strip()
            sfunc = func.split()
            
            fname = sfunc[1].split("(")[0].strip("*")
            farg = sfunc[1].split("(")[1].strip("*").strip(")").strip(";")
            sn_name = ""
            for (i, c) in enumerate(fname):
                if i == 0:
                    sn_name += c.lower()
                elif c.isupper():
                    sn_name += "_" +c.lower()
                else:
                    sn_name += c
        
            print(sn_name + ",")
                    
            # print("//", comment)
            # print("unsafe fn", sn_name + "(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> ", end="")
            # if sfunc[0] == "void" and farg == "void":
            #     print("{ raylib::ffi::"+fname+"(); None }")
            # elif sfunc[0] == "bool":
            #     print("{\n    // [0/1][@]\n    Some(vec![raylib::ffi::"+fname+"() as BaseType])\n}")
            # else:
            #     print("{ unimplemented!() }")
            # print()

if __name__ == "__main__":
    parse_input_maps()