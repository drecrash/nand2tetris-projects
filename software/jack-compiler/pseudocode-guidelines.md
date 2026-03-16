## ClassVarDec
`('static' | 'field') type varName (',' varName)* ';'`

```
For every variable in the line:

Push to class symbol table:
Symbol {
name = varName,
type = type,
kind = (static | field)
}
``` 

## SubroutineDec
`('constructor' | 'function' | 'method') ('void' | type) subroutineName '(' parameterList ')' subroutineBody``

```
*Case of method*

	Push to Subroutine Symbol Table:
		
		Symbol {
			name = this,
			type=className,
			kind="arg"
		}	
		
*Case of constructor*

	*Get # of fields ({n})*
	
*All cases*

	Write to VM:
	
		function {className}.{subroutineName} {nLcl}
	

*Case of constructor*
	Write to VM:
		push constant {n}
		call Memory.alloc 1
		pop pointer 0
		
*Case of method*
	Write to VM:
		push argument 0
		pop pointer 0
```

## SubroutineBody
`'{' varDec* statements '}'`
```


```
## varDec
`'var' type varName (',' varName)* ';'`
```
*for every variable in the declaration:*
	
	Push to Subroutine Symbol Table:
		Symbol {
			name = varName,
			type=type,
			kind="local"
		}
```

## Statement
`letStatement | ifStatement | whileStatement |doStatement | returnStatement`

## letStatement
`'let' varName ('[' expression ']')? '=' expression ';'`

```
*Get varName representation from stack:*
	Check if varName is in subroutine symbol table
	Check if varName is in class symbol table
	
	varNameRep = {type} {id} // ex: local 1
	
Write to VM:

	*Case with square brackets following varName, get index from given expression*
	
		push {varNameRep}
		push constant {index}
		add
	*Case with no square brackets*
		// do nothing
	
	*Calculate expression value, pushing the result onto the stack*
	
	*Case with varName[expression]:*
		// stack is now [variable base address + index (if applicable)]
		
		pop temp 0 // store the expression value
		pop pointer 1 // store the variable address
		push temp 0 // bring expression value back onto the stack
		pop that 0 // store the value into the address stored at 'pointer 1'
	
	*Case with no square brackets:*
		pop {varNameRep}
	
```

## ifStatement
`'if' '(' expression ')' '{' statements '}' ('else' '{' statements '}')?`
```
*Calculate expression, push to stack*

Write to VM:
	not
	if-goto L{i}

*Write initial statements to VM*

Write to VM:
	goto L{i+1}
	label L{i}

*Write else statements to VM*

Write to VM:
	label L{i+1}
	
*{i} should be incremented by 2 every time an if statement is made*
	
```

## whileStatement
`'while' '(' expression ')' '{' statements '}'`
```
Write to VM:
	label W{j}

*Calculate expression, pushing value to stack*

Write to VM:
	not
	if-goto W{j+1}
	
*Run statements*

Write to VM:
	goto W{j}
	label W{j+1}
	
*{j} should be incremented by 2 every time a while statement is written*
```

## doStatement
`'do' subroutineCall ';'`

## Return
`'return' expression? ';'`
```
*Calculate expression, pushing to stack*

Write to VM:
	pop temp 0
	return
```
## Expression
`term (op term)*`

## Term
`integerConstant | stringConstant | keywordConstant | varName | varName '[' expression ']' | subroutineCall | '(' expression ')' | unaryOp term`

```
*Case of integerConstant:*
	Write to VM:
		push constant {integerConstant}
	
*Case of stringConstant:*
	Write to VM:
		push constant {lengthOfString}
		call String.new 1
		
		*for every character in the string*
			push constant {charToAscii}
			call String.append 2
		
	
*Case of keywordConstant:*
	*Case "true":*
		Write to VM:
			push constant 1
			not
	
	*Case "false"*
		Write to VM:
			push constant 0
	
	*Case "null"*
		Write to VM:
			push constant 0
			
	*Case "this*
		Write to VM:
			push pointer 0
		

	
*Case of varName:*
	
	Get varName representation from stack:
		Check if varName is in subroutine symbol table
		Check if varName is in class symbol table
		
		varNameRep = {type} {id} // ex: local 1
	
	Write to VM:
		push {varNameRep}
		
*Case of varName[expression]*
	
	Write to VM:
		push {varNameRep}
	
	*Calculate expression value, pushing it onto the stack*
	
	Write to VM:
		add
		pop pointer 1 // make the 'that' pointer point to the memory address {varBase + expression}
		push that 0 // push the value pointed to
		
		
*Case of subroutineCall*
	
	*Call compileSubroutineCall*
	
*Case of (expression)*
	*Calculate expression value, pushing it onto the stack*
	
*Case of {unaryOp} {term}*
	*Calculate term value, pushing it onto the stack*
	
	*Case "-"*
		Write to VM:
			neg
	
	*Case "~"*
		Write to VM:
			not
```

## SubroutineCall
`subroutineName '(' expressionList ')' | (className | varName) '.' subroutineName '(' expressionList ')'`

```
*Case subroutineName(expressionList)*

	
	Write to VM:
		push pointer 0 // push 'this' to the stack because it's a method
		
	*Go through expressionList, pushing everything to the stack*
	*Have expressionList return the nArgs*
	
	Write to VM:
		call {currentClassName}.{subroutineName} {nArgs+1}
	
*Case (className | varName) '.' subroutineName '(' expressionList ')*

	*Case className*
		class_name = className
	*Case varName*
		
		Write to VM:
			class_name = {type of varName}
			push {varNameRep from symboltable}

	*Go through expressionList, pushing everything to the stack*
	*Have expressionList return the nArgs*
	
	Write to VM:
		call {class_name}.{subroutineName} {nArgs} // nArgs + 1 if varName and not className
```