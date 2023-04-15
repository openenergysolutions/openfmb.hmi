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
                "name": "SolarOn",
                "path": "SolarOn",
                "type": "command",
				"measurement": ""               
            }
        },
        {   
            "type": "element",
			"name": "Object",         
            "attributes": {
                "label": "",
                "name": "SolarOff",
                "path": "SolarOff",
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
                "name": "SetGgioValueAnalog",
                "path": "SetGgioValueAnalog",
                "type": "set-point",
				"measurement": ""               
            }
        }, 
		{ 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "SetGgioValueInteger",
                "path": "SetGgioValueInteger",
                "type": "set-point",
				"measurement": ""               
            }
        },
		{ 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "SetGgioValueBool",
                "path": "SetGgioValueBool",
                "type": "set-boolean",
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
        },
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "ANetMag",
				"path": "ANetMag",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "ANeutMag",
				"path": "ANeutMag",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "APhsAMag",
				"path": "APhsAMag",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "APhsBMag",
				"path": "APhsBMag",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "APhsCMag",
				"path": "APhsCMag",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "HzMag",
				"path": "HzMag",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "PfNetMag",
				"path": "PfNetMag",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "PfNeutMag",
				"path": "PfNeutMag",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "PfPhsAMag",
				"path": "PfPhsAMag",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "PfPhsBMag",
				"path": "PfPhsBMag",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "PfPhsCMag",
				"path": "PfPhsCMag",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "PhVNetAng",
				"path": "PhVNetAng",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "PhVNetMag",
				"path": "PhVNetMag",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "PhVNeutAng",
				"path": "PhVNeutAng",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "PhVNeutMag",
				"path": "PhVNeutMag",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "PhVPhsAAng",
				"path": "PhVPhsAAng",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "PhVPhsAMag",
				"path": "PhVPhsAMag",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "PhVPhsBAng",
				"path": "PhVPhsBAng",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "PhVPhsBMag",
				"path": "PhVPhsBMag",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "PhVPhsCAng",
				"path": "PhVPhsCAng",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "PhVPhsCMag",
				"path": "PhVPhsCMag",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "PpvPhsAbAng",
				"path": "PpvPhsAbAng",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "PpvPhsAbMag",
				"path": "PpvPhsAbMag",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "PpvPhsBcAng",
				"path": "PpvPhsBcAng",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "PpvPhsBcMag",
				"path": "PpvPhsBcMag",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "PpvPhsCaAng",
				"path": "PpvPhsCaAng",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "PpvPhsCaMag",
				"path": "PpvPhsCaMag",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "VaNetMag",
				"path": "VaNetMag",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "VaNeutMag",
				"path": "VaNeutMag",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "VaPhsAMag",
				"path": "VaPhsAMag",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "VaPhsBMag",
				"path": "VaPhsBMag",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "VaPhsCMag",
				"path": "VaPhsCMag",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "VArNetMag",
				"path": "VArNetMag",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "VArNeutMag",
				"path": "VArNeutMag",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "VArPhsAMag",
				"path": "VArPhsAMag",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "VArPhsBMag",
				"path": "VArPhsBMag",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "VArPhsCMag",
				"path": "VArPhsCMag",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "WNetMag",
				"path": "WNetMag",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "WNeutMag",
				"path": "WNeutMag",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "WPhsAMag",
				"path": "WPhsAMag",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "WPhsBMag",
				"path": "WPhsBMag",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "WPhsCMag",
				"path": "WPhsCMag",
				"type": "set-point",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "StartTransaction",
				"path": "StartTransaction",
				"type": "command",
				"measurement": ""
			}
		},
		{
			"type": "element",
			"name": "Object",
			"attributes": {
				"label": "",
				"name": "StopTransaction",
				"path": "StopTransaction",
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