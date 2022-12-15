#include "stm32f4xx.h"                  // Device header
#include "uart_driver.h"
#include "stdarg.h"
#include "string.h"

usart_handle_t uart_handle;
/*char message1[] = "ENTER: \n";
char message2[] = "AYESHA";
char message3[] = "ayesha.arif1014@gmail.com";
char message4[] = "no such information available";
*/
char      msg[]={0};

uint8_t rx_buffer[20]={0};

#define delay  for(int i=0;i<1000000;i++)

void uart_printf(char *format,...)
	{
		char buff[80];
		
		/*Extrating the argument list using VS apis*/
		va_list args;
		va_start(args,format);
		vsprintf(buff, format,args);
		
		for(int i=0;i< strlen(buff);i++){
		
		uart_handle.Instance->DR = buff[i];
		while(!(uart_handle.Instance->SR & USART_REG_SR_TXE_FLAG));
	}
	}
void rec_msg(char msg[]){
	if(msg[0] == 't'){
      GPIOC->ODR ^= (1<<13);	
			uart_printf("LED is toggled! \n");
	}
	else{
	uart_printf("invalid operation! \n");
	}

}	
//this function parses th command and takes action
/*void parse_cmd(char cmd[])
{
	if(cmd[0] =='N' && cmd[1] =='A' && cmd[2] =='M' && cmd[3] =='E')
	{
			//usart_tx(&uart_handle,message2,sizeof(message2)-1);
		uart_printf(message2);
	}
	else if(cmd[0] == 'E' && cmd[1] == 'M' && cmd[2] == 'A' && cmd[3]=='I')
	{
		//usart_tx(&uart_handle,message3,sizeof(message3)-1);
		uart_printf(message3);
	}
	else
	{
	//	while(uart_handle.tx_state != USART_STATE_READY);
	//	usart_tx(&uart_handle,message4, sizeof(message4)-1);
		uart_printf(message4);
	}

}
*/
//This callback will be called by the driver when driver finishes the transmission of data
static void app_tx_cmp_callback(void *size)
{
}

//This callback will be called by the driver when the application receives the command
static void app_rx_cmp_callback(void *size)
{
	//we got a command, parse it
//	parse_cmd(rx_buffer);
}



int main(void)
{
	
	RCC->AHB1ENR |= RCC_AHB1ENR_GPIOAEN | RCC_AHB1ENR_GPIOCEN;
	GPIOC->MODER |=(1<<26);
	GPIOC->ODR = 0;

	//TX using PA9
	GPIOA ->MODER |= (1<<19);
  GPIOA->AFR[1]|= (1<<4)|(1<<5)|(1<<6);
  
	//RX using PA10
	GPIOA ->MODER |= (1<<21);
  GPIOA->AFR[1]|= (0x07<<4) | (0x07<<8); //(1<<8)|(1<<9)|(1<<10);
	
	// enable clock for USART1 Peripheral
	_RCC_USART1_CLK_ENABLE();
	
	uart_handle.Instance = USART_1;
	
	uart_handle.Init.BaudRate     = USART_BAUD_9600;
	uart_handle.Init.WordLength   = USART_WL_1S8B;
	uart_handle.Init.StopBits     = USART_STOPBITS_1;
	uart_handle.Init.Parity       = USART_PARITY_NONE;
	uart_handle.Init.Mode         = USART_MODE_TX_RX;
//	uart_handle.Init.Mode         = USART_MODE_TX;
	uart_handle.Init.OverSampling = USART_OVER16_ENABLE;
	
	//fill out the application callbasks
	
	uart_handle.tx_cmp_cb   = app_tx_cmp_callback;
	uart_handle.rx_cmp_cb   = app_rx_cmp_callback;
	
	usart_init(&uart_handle);
	usart_handle_interrupt(&uart_handle);
	//enable the IRQ of USART1 Peripheral
	NVIC_EnableIRQ(34);
	
/*	while(uart_handle.tx_state != USART_STATE_READY);
	//Send the message
	usart_tx(&uart_handle,message1,sizeof(message1)-1);*/
	  usart_tx(&uart_handle,msg,5);
		usart_rx(&uart_handle, rx_buffer, 20);
		
	while(1)
	{

	//Send the message
//		uart_printf("A \n");
//    delay;
/*		if(uart_handle.Instance->SR & USART_REG_SR_RXNE_FLAG){
			char temp =  uart_handle.Instance-> DR;
			uart_handle.Instance->DR = temp;
			while(!(uart_handle.Instance->SR & USART_REG_SR_TXE_FLAG));
		}*/
		if(uart_handle.Instance->SR & USART_REG_SR_RXNE_FLAG){			
			char temp =  uart_handle.Instance-> DR;
			rec_msg(&temp);
		}
	}

}
