import sys

def minify(rbf_file, new_file):
    file = open(rbf_file, "r");
    f = open(new_file, "w+")
    no_comment_file = []
    for line in file:
        line = line.strip()
        if not line.startswith("//"):
            no_comment_file.append(line)
    for line in no_comment_file:
        for c in line:
            if c in "<>+-.,[]":
                f.write(c);
    file.close()
    f.close()
    
def unsigned(num, min_size, base):
    if num < 0: return "NULL"
    rems = []
    while num != 0:
        whole = num // base
        rem = num % base
        rems.append(rem)
        num = whole
    if len(rems) < min_size:
        for x in range(min_size - len(rems)):
            rems.append(0)
    rems.reverse()
    return rems

def twos_compliment(num, size, base):
    IMAX = ((base ** size) / 2) - 1
    IMIN = -((base ** size) / 2)
    if num > IMAX or num < IMIN:
        return "NULL"
    elif num < 0:
        UMAX = base ** size
        anti = UMAX + num # num = neg
        return unsigned(anti, size, base)
    else:
        return unsigned(num, size, base)

def base(num, b, size = 5):
    num_in_base = unsigned(num, size, b)
    signed = twos_compliment(num, size, b)
    print("unsigned:", num_in_base)
    print("signed:", signed)

def largest_factor_pair(number):
    max_factor = int( number**0.5) + 1
    largest_pair = (1, number)
    for i in range(2, max_factor):
        if number % i == 0:
            factor_pair = (i, number // i)
            largest_pair = max(largest_pair, factor_pair)
    return largest_pair

def rbf_gen(x, max_rem=10):
    code_list = []
    pair = largest_factor_pair(x)
    rbf = ">"
    rbf += "+" * pair[0]
    rbf += "[<"
    rbf += "+" * pair[1]
    rbf += ">-]"
    code_list.append(rbf)
    for i in range(1, max_rem+1):
        after_pair = largest_factor_pair(x+i)
        rbf = ">"
        rbf += "+" * after_pair[0]
        rbf += "[<"
        rbf += "+" * after_pair[1]
        rbf += ">-]"
        rbf += "<"
        rbf += "-" * i
        rbf += ">"
        code_list.append(rbf)
    for i in range(1, max_rem+1):
        before_pair = largest_factor_pair(x-i)
        rbf = ">"
        rbf += "+" * before_pair[0]
        rbf += "[<"
        rbf += "+" * before_pair[1]
        rbf += ">-]"
        rbf += "<"
        rbf += "+" * i
        rbf += ">"
        code_list.append(rbf)
    return min(code_list, key=len)

def text_to_bf(text):
    ascii_text = []
    for c in text:
        ascii_text.append(ord(c))
    for x in ascii_text:
        code = rbf_gen(x)
        print(code)


args = sys.argv
match len(args):
    case 4:
        val = args[1]
        command = args[2]
        val2 = args[3]
        match command:
            case "minify":
                minify(val, val2)
            case "base":
                base(int(val), int(val2))
            case "repeat":
                print("char:", val)
                print(val * int(val2))
            case _:
                print(args)
                print("Command doesnt exist!")
                print("try 'commands'.")
    case 3:
        command = args[1]
        val = args[2]
        match command:
            case "bf":
                if val.isnumeric():
                    print(rbf_gen(int(val)))
                elif type(val) == str:
                    text_to_bf(val)
                else:
                    print("Wrong type for arg!")
            case "count":
                print("'"+val+"'")
                print("characters:", len(val))
            case _:
                print(args)
                print("Command doesnt exist!")
                print("try 'commands'.")
    case 2:
        command = args[1]
        match command:
            case "commands":
                print("commands: list commands")
                print("minify [x, c, x]: shorten bf file to another file")
                print("base [x, c, x]: convert num to base n")
                print("bf [c, x]: generate bf code for num/string")
                print("count [c, x]: count amount of chars in str")
                print("repeat [x, c, x]: repeat char n amount")
                print("x = value, c = command")
            case _:
                print(args)
                print("Command doesnt exist!")
                print("try 'commands'.")
    case _:
        print("Incorrect arg amount!")
        exit()   
            
