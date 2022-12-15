#include "stm32f4xx.h"                  // Device header
#include "stdint.h"
#include "string.h"
#include "stdlib.h"
#include "stdarg.h"

#define delay  for(int i=0;i<10000000;i++)
//uint32_t HSE_VALUE = 8000000;

void systm_clk_cnfg(void);
void print_msg(char *msg,...);


int main(void){
	
	systm_clk_cnfg();
	
	RCC->AHB1ENR |= RCC_AHB1ENR_GPIOAEN | RCC_AHB1ENR_GPIOCEN;
	RCC->APB2ENR |= RCC_APB2ENR_USART1EN;
	
	//GPIOA->OTYPER &= ~(0xFFFFFFFF); //resetting the output type reg. for push-pull config
	GPIOA->MODER |= (1<<19) | (1<<21); //setting the PA9 & PA10 to 546.875lt. func.
	GPIOA->AFR[1] |= (0x07<<4) | (0x07<<8); //setting the USART1_RX and USART1_TX to GPI0A_AFRH
	
	GPIOC->MODER |= (1<<26);
	GPIOC->ODR = 0;
	
	USART1->BRR |= 0x222E; //setting the baud rate reg value to 546.875
	USART1->CR1 |= USART_CR1_TE; //enabling the transmit bit in USART_CR1 reg
	USART1->CR1 |= USART_CR1_UE;
	USART1->CR1 |= USART_CR1_RE;
	
	while(1){
		
		print_msg("256\n");
		delay;
		
/*		if(USART1->SR & USART_SR_RXNE){
      GPIOC->ODR ^= (1<<13);			
			char temp =  USART1-> DR;
			USART1->DR = temp;
			while(!(USART1->SR & USART_SR_TXE));
		}*/
	}
}

	
void print_msg(char *msg,...){
	
	char buff[80];
	
	va_list args;
	va_start(args,msg); //extracting and listing all the arguments for user string
	vsprintf(buff,msg,args);
	
	for(int i=0;i< strlen(buff);i++){
		
		USART1->DR = buff[i];
		while(!(USART1->SR & USART_SR_TXE));
	}
}

void systm_clk_cnfg(void){
	
	/*Switching to HSE and setting the PLL source to HSE*/
	
	//setting the external clock ON
	RCC->CR |= RCC_CR_HSEON;
	//Waiting while the HSE is stable 
	while(!(RCC->CR & RCC_CR_HSERDY));
	
	// activate flash buffer 
	FLASH->ACR |= FLASH_ACR_PRFTEN;
	
	//FLASH wait state
	FLASH->ACR &= ~ FLASH_ACR_LATENCY;
	FLASH->ACR |= FLASH_ACR_LATENCY_1WS;  //
	
	//Setting HSE as the PLL clock source 
	RCC->PLLCFGR |= RCC_PLLCFGR_PLLSRC;
	
	/*Configuring the value of PLL values for different periphs*/
	
	/*configuring the PLL_M value*/
	RCC->PLLCFGR &= ~RCC_PLLCFGR_PLLM_Msk; //Masking the first 6 bits of PLLCFGR which is PLL_M
	RCC->PLLCFGR |= (0x19<<RCC_PLLCFGR_PLLM_Pos); //seting the PLL_M value to 25
	
	/*Now configuring the PLL_N value*/
	RCC->PLLCFGR &= ~RCC_PLLCFGR_PLLN_Msk; //Masking the first PLL_N bits of PLLCFGR Register
	RCC->PLLCFGR |= (0xA8<<RCC_PLLCFGR_PLLN_Pos); //seting the PLL_N value to 168
	
	/*Now configuring the PLL_P value*/
	RCC->PLLCFGR &= ~RCC_PLLCFGR_PLLP_Msk; //Masking the first PLL_P bits of PLLCFGR Register
	RCC->PLLCFGR |= (0x00<<RCC_PLLCFGR_PLLP_Pos); //seting the PLL_P value to 2
	
	
	/*Now configuring AHB,APB1 and APB2 clocks*/
	
	/*AHB prescaling to 84Mhz*/
	RCC->CFGR &= ~RCC_CFGR_HPRE_Msk; //Masking the AHB presc, and leavin it zero since no division needed
	
	/*APB1 prescaling to 42Mhz*/
	RCC->CFGR &= ~RCC_CFGR_PPRE1_Msk;
	RCC->CFGR |= (0x04<<RCC_CFGR_PPRE1_Pos); //prescaling to /2 by setting 100 on PRE1 bits
	

	/*APB2 prescaling to 84Mhz*/
	RCC->CFGR &= ~RCC_CFGR_PPRE2_Msk; //Masking and leaving at zero since APB2 runs at 84MHz	
  
	/*Now enabling the PLL clokc*/
	RCC->CR |= RCC_CR_PLLON;
	while(!(RCC->CR & RCC_CR_PLLRDY)); //wait until the PLL is stable and locked

	
	/*Now making PLL as the SYSCLCK(system clock)*/
	
	RCC->CFGR &= ~RCC_CFGR_SW;
	RCC->CFGR |= RCC_CFGR_SW_PLL; //setting the SW bits to 0x02=01 for PLL selection
	while(!(RCC->CFGR & RCC_CFGR_SWS_PLL));
	
	//SystemCoreClockUpdate();
	
	
}
