def largest_factor_pair(number):
    max_factor = int( number**0.5) + 1
    largest_pair = (1, number)

    for i in range(2, max_factor):
        if number % i == 0:
            factor_pair = (i, number // i)
            largest_pair = max(largest_pair, factor_pair)

    return largest_pair

def rbf_gen(x, max_rem):
    
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
        code = rbf_gen(x, 10)
        print(code)
    


text_to_bf("Congrats! You created your first window!")
