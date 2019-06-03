# Readme

A functional toy language (WIP)

## Syntax

Assignments
```
a = 1,
b = a + 1,
```
Functions
```
f = \x.x+1;

f(3.0),

# Recursive and nested functions are allowed
fib \n.
 fib2 = \a,b,m.(m==0)*a + (m!=0)*fib2(b, a+b, m-1);
 fib2(0,1,n);

fib(3),
```
Hello World!
```
is = \ptr,do.
 (ptr!=0)*do;

print = \ptr.
 put_not_null = \ptr.
  a = _load(ptr),
  is(a, _put(a)),
  is(a, put_not_null(ptr+1));
 _put(\n);

store4 = \ptr,a,b,c,d.
 _store(ptr  ,a),
 _store(ptr+1,b),
 _store(ptr+2,c),
 _store(ptr+3,d);
 
hello_world = _alloc(12),
store4(hello_world,      'H','e','l','l'),
store4(hello_world + 1, 'o',' ','W','o'),
store4(hello_world + 2, 'r','l','d','!'),

print(hello_world),
```

## Features
* Closures
* Immutability
* Minimal syntax
* Function scoping
* Implicit numerical conversion
* Recursive

## Builtin functions
### Put
Puts a number to the console,
if the number is in the printable ascii range it will be represented with an ascii character,
otherwise the function is undefined.
```
_put = \c;
```
### Get
Reads that last character from stdin and returns it by its numerical value. If stdin is empty
-1 will be returned.
```
_get = \.c;
```
### Alloc
Allocates memory of size _s_. The function returns a pointer to the allocated memory.
```
_alloc = \s.p;
```
### Realloc
Reallocates memory of size _s_ and pointer _p_. The function returns a pointer to the reallocated memory.
```
_realloc = \p,s.p;
```
### Free
Frees the memory held by a pointer.
```
_free = \p;
```
### Store
Stores value _v_ at pointer _p_. 
```
_store = \p,v;
```
### Load
Loads value _v_ at pointer _p_.
```
_load = \p.v;
```