#include "stm32f4xx_hal.h"              // Keil::Device:STM32Cube HAL:Common
#include "arm_math.h"                   // ARM::CMSIS:DSP

#define	SIG_LENGTH		320
#define IMP_RSP_LENGTH		29
extern void SystemClock_Config(void);
extern float32_t inputSignal_f32_1kHz_15kHz[SIG_LENGTH];
extern  float32_t impulse_response [IMP_RSP_LENGTH];
float32_t outputSignal_arr[SIG_LENGTH + IMP_RSP_LENGTH];


void plot_input_signal(void);
float32_t signal_mean(float32_t *sig_src_arr,uint32_t sig_length);

float32_t inputSample;
float32_t imp_rspSample;
float32_t outputSample;
void plot_imp_response(void);
void plot_outout_signal(void);
void plot_all(void);

uint32_t freq;

int main()
{
	HAL_Init();
	SystemClock_Config();
	freq = HAL_RCC_GetHCLKFreq();
	
    
	   arm_conv_f32(&inputSignal_f32_1kHz_15kHz[0],
									SIG_LENGTH,
									&impulse_response[0],
	                IMP_RSP_LENGTH,
									&outputSignal_arr[0]
	                );
	
 plot_all();

	while(1)
	{
	}
}


void plot_all(void)
{
	int i,j,g,k;
	i=j=0;
	 for(k=0;k<(SIG_LENGTH+IMP_RSP_LENGTH);k++)
	{
		 i++;
		 j++;
		if(i==SIG_LENGTH) i=0;
		if(j==IMP_RSP_LENGTH) j=0;
		if(k==(IMP_RSP_LENGTH+SIG_LENGTH)-1)k=0;
		outputSample = outputSignal_arr[k];
		inputSample  = inputSignal_f32_1kHz_15kHz[i];
		imp_rspSample = impulse_response[j];
		
		for(g=0;g<20000;g++){}
	}
	
}
void plot_outout_signal(void)
{
	int i,j;
	for(i=0;i<(SIG_LENGTH+IMP_RSP_LENGTH);i++)
	{
		outputSample =outputSignal_arr[i];
		for(j=0;j<3000;j++){}
	}
	
}



void plot_input_signal(void)
{
	int i,j;
	for(i=0;i<SIG_LENGTH;i++)
	{
		inputSample =inputSignal_f32_1kHz_15kHz[i];
		for(j=0;j<3000;j++){}
	}
	
}

void plot_imp_response(void)
{
	 int i,j;
	 for(i=0;i<IMP_RSP_LENGTH;i++)
	{
		 imp_rspSample  = impulse_response[i];
		for(j=0;j<3000;j++){}
	}
}



void SysTick_Handler(void)
{
	 HAL_IncTick();
	 HAL_SYSTICK_IRQHandler();
	
}