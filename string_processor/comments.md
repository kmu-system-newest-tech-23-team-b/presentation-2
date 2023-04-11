# 문자열 처리 클래스

## 설명

StringProcessor struct 구조체

## main 설명

1. 메인 함수에서는 먼저 사용자 입력을 받습니다.
2. 사용자 입력을 기반으로 StringProcessor 객체를 생성합니다.
3. 각 메소드를 호출하여 결과를 출력합니다.

## StringProcessor 설명

1. input: String 보유하고 있는 StringProcessor 구조체 정의합니다. 
2. new, upper, lower, len, find 메소드를 정의한 구조체에 대한 impl 정의합니다. 
3. new 메소드는 인스턴스를 생성하는 생성자이므로 입력 문자열을 받아서 넘깁니다. (StringProcessor { input })
4. to_uppercase 메소드는 입력 문자열을 대문자로 변환하여 반환하도록 구현합니다. (self.input.to_uppercase())
5. to_lowercase 메소드는 입력 문자열을 소문자로 변환하여 반환하도록 구현합니다. (self.input.to_lowercase())
6. len 메소드는 입력 문자열의 길이를 반환하도록 구현합니다. (self.input.len())
7. find 메소드는 입력 문자열에서 특정 문자를 찾아서 위치를 반환합니다. (self.input.find(c))

