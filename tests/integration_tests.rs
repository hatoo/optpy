use optpy_std::Value;
use optpy_test_macro::test_python;

mod test_env;

macro_rules! optpy_integration_test {
    ($name:ident, $code:expr, $(($input:expr, $output:expr)),+) => {
        #[test]
        fn $name() {
            let code = $code;
            let tests = [$(($input, $output)),+];
            for (input, expected) in tests {
                let (output, code) = test_env::execute(code, input).unwrap();
                assert_eq!(output, expected, "{}", code);
            }
        }
    };
    (ignore, $name:ident, $code:expr, $(($input:expr, $output:expr)),+) => {
        #[test]
        #[ignore]
        fn $name() {
            let code = $code;
            let tests = [$(($input, $output)),+];
            for (input, expected) in tests {
                let (output, code) = test_env::execute(code, input).unwrap();
                assert_eq!(output, expected, "{}", code);
            }
        }
    };
}

optpy_integration_test! {
test_if_statement,
r#"
a, b = map(int, input().split())
ans = a * b
if ans % 2 == 0:
    print("Even")
else:
    print("Odd")
"#,
("3 4\n", "Even\n"),
("3 5\n", "Odd\n")
}

optpy_integration_test! {
test_multiple_if_conditions,
r#"
a, b, c = map(int, input().split())
ans = a * b
if a <= b < c:
    print("IN")
else:
    print("OUT")
"#,
("3 4 5\n", "IN\n"),
("3 5 4\n", "OUT\n")
}

optpy_integration_test! {
test_list_add_assign,
r"
A = list(map(int, input().split()))
A[0] += 1
print(A[0])
",
("1 2 3\n", "2\n")
}

optpy_integration_test! {
test_solve_abc081_b,
r#"
N = int(input())
A = list(map(int, input().split()))

flag = 0
count = 0

while True:
    for i in range(N):
        if A[i] % 2 != 0:
            flag = 1
    if flag == 1:
        break
    for i in range(N):
        A[i] = A[i]//2
    count += 1
print(count)
"#,
("3\n8 12 40\n", "2\n"),
("4\n5 6 8 10\n", "0\n")
}

optpy_integration_test! {
test_for_loop,
r#"
N = int(input())
ans = 0
for i in range(N):
    ans += i
print(ans)
"#,
("5\n", "10\n"),
("10\n", "45\n")
}

optpy_integration_test! {
test_recursive_fibonacci,
r#"
def fib(n):
    if n == 1 or n == 0:
        return 1
    return fib(n - 1) + fib(n - 2)
n = int(input())
n = fib(n)
print(n)
"#,
("0\n", "1\n"),
("1\n", "1\n"),
("2\n", "2\n"),
("3\n", "3\n"),
("4\n", "5\n")
}

optpy_integration_test! {
test_multiple_print,
r#"
A = 1
B = 2
S = "hello"
print(A+B, S)
"#,
("", "3 hello\n")
}

optpy_integration_test! {
test_list_initialization,
r#"
A = []
A.append("A")
A.append("B")
print(A[0], A[1])
"#,
("", "A B\n")
}

optpy_integration_test! {
test_tuple_for_target,
r#"
A = [["A", "B"] , ["C", "D"]]
for a, b in A:
    print(b, a)
"#,
("", "B A\nD C\n")
}

optpy_integration_test! {
test_assign_self,
r#"
x = [0]
x[0] = x[0]
print(x[0])
"#,
("", "0\n")
}

optpy_integration_test! {
test_assign_in_loop,
r#"
for i in [0, 1, 2]:
    x = i
print(x)
"#,
("", "2\n")
}

optpy_integration_test! {
test_mutate_argument,
r#"
def f(arr):
    arr[0] = 200
arr = [0]
f(arr)
print(arr[0])
"#,
("", "200\n")
}

optpy_integration_test! {
test_short_circuit_evaluation,
r#"
def a():
    print("eval")
    return True

N = int(input())
if N == 1 and a():
    print("YES")
else:
    print("NO")
"#,
("0\n", "NO\n"),
("1\n", "eval\nYES\n")
}

optpy_integration_test! {
test_array_assignment,
r#"
a = [0, 1, 2]
a[0] = a[1]
a[1] = a[2]
print(a[0], a[1], a[2])
"#,
("", "1 2 2\n")
}

#[test]
fn test_ops() {
    let result = test_python!(
        r"
a = 1
b = 2
return a+b"
    );
    assert_eq!(result, Value::from(3));

    let result = test_python!(
        r"
a=2
b=4
return a*b
"
    );
    assert_eq!(result, Value::from(8));

    let result = test_python!(
        r"
a=2
b=4
return a-b
"
    );
    assert_eq!(result, Value::from(-2));

    let result = test_python!(
        r"
a=2
b=4
return a/b
"
    );
    assert_eq!(result, Value::from(0.5));
}
