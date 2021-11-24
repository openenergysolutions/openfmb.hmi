// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

export const getCommands = () => {
    return Object.keys(COMMANDS);
};
  
export const getCommandsByType = (type: string) => {
    return COMMANDS[type] ? COMMANDS[type] : []
};

export const COMMANDS = {
    "MicrogridControl": [        
        {   
            "type": "element",
			"name": "Object",         
            "attributes": {
                "label": "",
                "name": "ResetDevices",
                "path": "ResetDevices",
                "type": "command",
				"measurement": ""               
            }
        }, 
        {   
            "type": "element",
			"name": "Object",         
            "attributes": {
                "label": "",
                "name": "InitiateIsland",
                "path": "InitiateIsland",
                "type": "command",
				"measurement": ""               
            }
        },        
        {   
            "type": "element",
			"name": "Object",         
            "attributes": {
                "label": "",
                "name": "InitiateGridConnect",
                "path": "InitiateGridConnect",
                "type": "command",
				"measurement": ""               
            }
        }, 
        {   
            "type": "element",
			"name": "Object",         
            "attributes": {
                "label": "",
                "name": "PccControl",
                "path": "PccControl",
                "type": "command",
				"measurement": ""               
            }
        },
        {   
            "type": "element",
			"name": "Object",          
            "attributes": {
                "label": "",
                "name": "EnableNetZero",
                "path": "EnableNetZero",
                "type": "command",
				"measurement": ""               
            }
        }, 
        {   
            "type": "element",
			"name": "Object",          
            "attributes": {
                "label": "",
                "name": "DisableNetZero",
                "path": "DisableNetZero",
                "type": "command",
				"measurement": ""               
            }
        }                  
    ],
    "DeviceControl": [
        {   
            "type": "element",
			"name": "Object",         
            "attributes": {
                "label": "",
                "name": "EnableSolarInverter",
                "path": "EnableSolarInverter",
                "type": "command",
				"measurement": ""               
            }
        },
        {   
            "type": "element",
			"name": "Object",         
            "attributes": {
                "label": "",
                "name": "DisableSolarInverter",
                "path": "DisableSolarInverter",
                "type": "command",
				"measurement": ""               
            }
        },
        {    
            "type": "element",
			"name": "Object",        
            "attributes": {
                "label": "",
                "name": "EnableLoadbank",
                "path": "EnableLoadbank",
                "type": "command",
				"measurement": ""               
            }
        },
        {     
            "type": "element",
			"name": "Object",       
            "attributes": {
                "label": "",
                "name": "DisableLoadbank",
                "path": "DisableLoadbank",
                "type": "command",
				"measurement": ""               
            }
        },
        {   
            "type": "element",
			"name": "Object",         
            "attributes": {
                "label": "",
                "name": "EssStart",
                "path": "EssStart",
                "type": "command",
				"measurement": ""               
            }
        },
        {    
            "type": "element",
			"name": "Object",        
            "attributes": {
                "label": "",
                "name": "EssDischarge",
                "path": "EssDischarge",
                "type": "command",
				"measurement": ""               
            }
        },
        {    
            "type": "element",
			"name": "Object",        
            "attributes": {
                "label": "",
                "name": "EssSocManage",
                "path": "EssSocManage",
                "type": "command",
				"measurement": ""               
            }
        },
        {    
            "type": "element",
			"name": "Object",        
            "attributes": {
                "label": "",
                "name": "EssSocLimits",
                "path": "EssSocLimits",
                "type": "command",
				"measurement": ""               
            }
        },         
        {    
            "type": "element",
			"name": "Object",        
            "attributes": {
                "label": "",
                "name": "EssStop",
                "path": "EssStop",
                "type": "command",
				"measurement": ""               
            }
        },
        {     
            "type": "element",
			"name": "Object",       
            "attributes": {
                "label": "",
                "name": "GeneratorOn",
                "path": "GeneratorOn",
                "type": "command",
				"measurement": ""               
            }
        },
        {    
            "type": "element",
			"name": "Object",        
            "attributes": {
                "label": "",
                "name": "GeneratorDisabled",
                "path": "GeneratorDisabled",
                "type": "command",
				"measurement": ""               
            }
        },
        {    
            "type": "element",
			"name": "Object",        
            "attributes": {
                "label": "",
                "name": "GeneratorEnabled",
                "path": "GeneratorEnabled",
                "type": "command",
				"measurement": ""               
            }
        },
        {     
            "type": "element",
			"name": "Object",       
            "attributes": {
                "label": "",
                "name": "GeneratorOff",
                "path": "GeneratorOff",
                "type": "command",
				"measurement": ""               
            }
        },
        {      
            "type": "element",
			"name": "Object",      
            "attributes": {
                "label": "",
                "name": "SwitchOneOpen",
                "path": "SwitchOneOpen",
                "type": "command",
				"measurement": ""               
            }
        },
        {    
            "type": "element",
			"name": "Object",        
            "attributes": {
                "label": "",
                "name": "SwitchOneClosed",
                "path": "SwitchOneClosed",
                "type": "command",
				"measurement": ""               
            }
        },        
        {     
            "type": "element",
			"name": "Object",       
            "attributes": {
                "label": "",
                "name": "SwitchTwoOpen",
                "path": "SwitchTwoOpen",
                "type": "command",
				"measurement": ""               
            }
        },
        {     
            "type": "element",
			"name": "Object",       
            "attributes": {
                "label": "",
                "name": "SwitchTwoClosed",
                "path": "SwitchTwoClosed",
                "type": "command",
				"measurement": ""               
            }
        },        
        {      
            "type": "element",
			"name": "Object",      
            "attributes": {
                "label": "",
                "name": "BreakerThreeOpen",
                "path": "BreakerThreeOpen",
                "type": "command",
				"measurement": ""               
            }
        },
        {    
            "type": "element",
			"name": "Object",        
            "attributes": {
                "label": "",
                "name": "BreakerThreeClosed",
                "path": "BreakerThreeClosed",
                "type": "command",
				"measurement": ""               
            }
        },        
        {    
            "type": "element",
			"name": "Object",        
            "attributes": {
                "label": "",
                "name": "SwitchFourOpen",
                "path": "SwitchFourOpen",
                "type": "command",
				"measurement": ""               
            }
        },
        { 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "SwitchFourClosed",
                "path": "SwitchFourClosed",
                "type": "command",
				"measurement": ""               
            }
        },
        { 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "ResetSwitchOne",
                "path": "ResetSwitchOne",
                "type": "command",
				"measurement": ""               
            }
        },
        { 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "ResetSwitchTwo",
                "path": "ResetSwitchTwo",
                "type": "command",
				"measurement": ""               
            }
        },
        { 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "ResetSwitchThree",
                "path": "ResetSwitchThree",
                "type": "command",
				"measurement": ""               
            }
        },
        { 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "ResetSwitchFour",
                "path": "ResetSwitchFour",
                "type": "command",
				"measurement": ""               
            }
        },
        { 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "ResetBreakerThree",
                "path": "ResetBreakerThree",
                "type": "command",
				"measurement": ""               
            }
        },
        { 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "ResetBreaker",
                "path": "ResetBreaker",
                "type": "command",
				"measurement": ""               
            }
        },
        { 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "ResetEss",
                "path": "ResetEss",
                "type": "command",
				"measurement": ""               
            }
        },
        { 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "ResetSolar",
                "path": "ResetSolar",
                "type": "command",
				"measurement": ""               
            }
        },
        { 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "ResetLoadbank",
                "path": "ResetLoadbank",
                "type": "command",
				"measurement": ""               
            }
        },
        { 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "ResetLoad",
                "path": "ResetLoad",
                "type": "command",
				"measurement": ""               
            }
        },
        { 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "SetModBlkOn",
                "path": "SetModBlkOn",
                "type": "command",
				"measurement": ""               
            }
        },
        { 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "SetModBlkOff",
                "path": "SetModBlkOff",
                "type": "command",
				"measurement": ""               
            }
        },
        { 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "SetWNetMag",
                "path": "SetWNetMag",
                "type": "set-point",
				"measurement": ""               
            }
        }, 
        { 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "SetVarNetMag",
                "path": "SetVarNetMag",
                "type": "set-point",
				"measurement": ""               
            }
        },
        { 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "SetValue",
                "path": "SetValue",
                "type": "set-point",
				"measurement": ""               
            }
        },            
        { 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "BlackStartEnable",
                "path": "BlackStartEnable",
                "type": "command",
				"measurement": ""               
            }
        },
        { 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "BlackStartDisable",
                "path": "BlackStartDisable",
                "type": "command",
				"measurement": ""               
            }
        },
        { 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "FrequencySetPointEnable",
                "path": "FrequencySetPointEnable",
                "type": "command",
				"measurement": ""               
            }
        },
        { 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "FrequencySetPointDisable",
                "path": "FrequencySetPointDisable",
                "type": "command",
				"measurement": ""               
            }
        },
        { 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "ReactivePowerSetPointEnable",
                "path": "ReactivePowerSetPointEnable",
                "type": "command",
				"measurement": ""               
            }
        },
        { 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "ReactivePowerSetPointDisable",
                "path": "ReactivePowerSetPointDisable",
                "type": "command",
				"measurement": ""               
            }
        },
        { 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "RealPowerSetPointEnable",
                "path": "RealPowerSetPointEnable",
                "type": "command",
				"measurement": ""               
            }
        },
        { 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "RealPowerSetPointDisable",
                "path": "RealPowerSetPointDisable",
                "type": "command",
				"measurement": ""               
            }
        },
        { 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "TransToIslandOnGridLossEnable",
                "path": "TransToIslandOnGridLossEnable",
                "type": "command",
				"measurement": ""               
            }
        },
        { 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "TransToIslandOnGridLossDisable",
                "path": "TransToIslandOnGridLossDisable",
                "type": "command",
				"measurement": ""               
            }
        },
        { 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "VoltageSetPointEnable",
                "path": "VoltageSetPointEnable",
                "type": "command",
				"measurement": ""               
            }
        },
        { 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "VoltageSetPointDisable",
                "path": "VoltageSetPointDisable",
                "type": "command",
				"measurement": ""               
            }
        },
        { 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "SetStateKindUndefined",
                "path": "SetStateKindUndefined",
                "type": "command",
				"measurement": ""               
            }
        },
        { 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "SetStateKindOff",
                "path": "SetStateKindOff",
                "type": "command",
				"measurement": ""               
            }
        },
        { 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "SetStateKindOn",
                "path": "SetStateKindOn",
                "type": "command",
				"measurement": ""               
            }
        },
        { 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "SetStateKindStandBy",
                "path": "SetStateKindStandBy",
                "type": "command",
				"measurement": ""               
            }
        }
    ],
    "HmiControl": [
        {   
            "type": "element",
			"name": "Object",         
            "attributes": {
                "label": "",
                "name": "ToggleEnvironment",
                "path": "ToggleEnvironment",
                "type": "command",
				"measurement": ""               
            }
        }
    ]
};