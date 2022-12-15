; programming cortex_m4 to solve simple equiation
; P = Q + R + S
; where Q=10, R=20, S=15

Q		EQU		10
R		EQU		20
S		EQU		15
	
		AREA simp_equ,CODE,READONLY
		ENTRY
		EXPORT __main

__main
		MOV	R1,#Q
		MOV	R2,#R
		MOV	R3,#S
		
		ADD	R0,R1,R2
		ADD R0,R0,R3

__stop	B	__stop

		END