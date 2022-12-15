#include "stm32f4xx.h"

int main(){
	
	//initialize the port c clock
	RCC->AHB1ENR |= (1<<2) | (1<<0);
	
	//configure pin13
	GPIOC->MODER |= (1<<26);
	GPIOC->OTYPER &= ~(1<<13);
	
	//configuring PA0
	GPIOA->MODER |= 0x00;
	GPIOA->PUPDR |= (1<<0);
	
	GPIOC->BSRR |= (1<<29);
	
	while(1){
		
		if((GPIOA->IDR & 0x01)==0)
		{
			GPIOC->ODR &= ~(1<<13);
			
		}else{
			
			GPIOC->ODR |= (1<<13);
		}
	}
}
