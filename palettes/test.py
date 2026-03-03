def divide_256_into(num):
    for i in range(num+1):
        p = int((256/num) * i)
        if p == 256:
            p = 255
        print(f"#{p:02X}{p:02X}{p:02X}")

if __name__=="__main__":

    divide_256_into(4)
