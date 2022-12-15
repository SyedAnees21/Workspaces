#include "stm32f4xx.h"                  // Device header

void TIM4_IRQHandler(void);
void delay_us(int);

int myticks = 0;
	

int main (){
	
	RCC->APB1ENR |= (1<<2);
	RCC->AHB1ENR |= (1<<2);
	
		
	GPIOC->MODER |= (1<<26);
	GPIOC->OTYPER &= ~(1<<13);
	GPIOC->ODR |= (1<<13);
	
	TIM4->PSC = 0;
	TIM4->ARR = 42;
	TIM4->CR1 |= TIM_CR1_CMS | TIM_CR1_CKD;
	TIM4->CR1 |= TIM_CR1_URS;
	TIM4->DIER |= TIM_DIER_UIE;
	TIM4->EGR |= TIM_EGR_UG;
	
	NVIC_EnableIRQ(TIM4_IRQn);
	
	while(1){
		
		GPIOC->ODR |= (1<<13);
		delay_us(1000000);
		GPIOC->ODR &= ~(1<<13);
		delay_us(1000000);
	}
}

void TIM4_IRQHandler(void){
	
	myticks++;
	TIM4->SR &= ~TIM_SR_UIF;
}

void delay_us(int Us){
	
	myticks=0;
	TIM4->CR1 |= TIM_CR1_CEN;
	while(myticks< Us);
	TIM4->CR1 &= ~TIM_CR1_CEN;	
	
}
