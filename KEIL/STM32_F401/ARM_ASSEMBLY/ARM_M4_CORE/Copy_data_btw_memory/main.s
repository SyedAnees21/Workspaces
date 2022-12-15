RAM1_ADDR	EQU		0x20000000
RAM2_ADDR	EQU		0x20000100

			AREA source,CODE,READONLY
			ENTRY
			EXPORT __main
				
__main
			BL		__fill
			BL		__copy
				
__stop		B		__stop

__fill
			LDR		R1,=RAM1_ADDR
			MOV		R0,#10
			LDR		R2,=0xDEADBEEF	;0xDEADBEEF is an hex num
	
__loop1
			STR		R2,[R1]
			ADD		R1,R1,#4
			SUBS	R0,R0,#1
			BNE		__loop1
			BX		LR
			
__copy
			LDR		R1,=RAM1_ADDR
			LDR		R2,=RAM2_ADDR
			MOV		R0,#10
			
__loop2
			LDR		R3,[R1]
			STR		R3,[R2]
			ADD		R1,R1,#4
			ADD		R2,R2,#4
			SUBS	R0,R0,#1
			BNE		__loop2
			BX		LR
			
			END