#include "systm_clock_config.h"

void systm_clk_cnfg(void){
	
	/*Switching to HSE and setting the PLL source to HSE*/
	
	//setting the external clock ON
	RCC->CR |= RCC_CR_HSEON;
	//Waiting while the HSE is stable 
	while(!(RCC->CR & RCC_CR_HSERDY));
	//disabling the PLL source
	RCC->CR &= ~RCC_CR_PLLON;
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
	RCC->PLLCFGR |= (0x00<<RCC_PLLCFGR_PLLN_Pos); //seting the PLL_P value to 2
	
	/*Now enabling the PLL clokc*/
	RCC->CR |= RCC_CR_PLLON;
	while(!(RCC->CR & RCC_CR_PLLRDY)); //wait until the PLL is stable and locked
	
	/*Now making PLL as the SYSCLCK(system clock)*/
	
	RCC->CFGR &= ~RCC_CFGR_SW_Msk;
	RCC->CFGR &= RCC_CFGR_SW_PLL; //setting the SW bits to 0x02=01 for PLL selection
	
	/*Now configuring AHB,APB1 and APB2 clocks*/
	
	/*AHB prescaling to 84Mhz*/
	RCC->CFGR &= ~RCC_CFGR_HPRE_Msk; //Masking the AHB presc, and leavin it zero since no division needed
	
	/*APB1 prescaling to 42Mhz*/
	RCC->CFGR &= ~RCC_CFGR_PPRE1_Msk;
	RCC->CFGR |= (0x04<<RCC_CFGR_PPRE1_Pos); //prescaling to /2 by setting 100 on PRE1 bits
	
	/*APB2 prescaling to 84Mhz*/
	RCC->CFGR &= ~RCC_CFGR_PPRE2_Msk; //Masking and leaving at zero since APB2 runs at 84MHz
	
}
