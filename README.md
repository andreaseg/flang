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

f(3.0)

fib \n.
 fib2 = \a,b,m.(m==0)*a + (m!=0)*fib2(b, a+b, m-1);
 fib2(0,1,n);

fib(3)
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
_get = c,
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