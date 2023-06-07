def encrypt(text, shift):
    code_a = ord('A')
    code_z = ord('Z')

    result = ""

    for ch in text:
        #문자 코드로 변환? 아스키코드?
        code = ord(ch)

        if code_a <= code <= code_z:
            #shift 만큼 뒤의 문자
            code = (code - code_a + shift) % 26 + code_a
        #문자로 변환
        result += chr(code)
    return result
#암호화
enc = encrypt("I lOVE RUST.", 3)
#복호화
dec = encrypt(enc, -3)
print(enc, "=>", dec)