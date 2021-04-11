# asciiart.py

for y in range(-12,12):
    for x in range(-39,39):
        ca=x*0.0458
        cb=y*0.08333
        a=ca
        b=cb
        for i in range(0,16):
            t=a*a-b*b+ca
            b=2*a*b+cb
            a=t
            flag=0
            if (a*a+b*b)>4:
                flag = 1
                break
        if flag == 0:
            print(" ", end="")
        else:
            if i>9:
                i=i+7
            print(chr(48+i), end="")
    print(" ")
