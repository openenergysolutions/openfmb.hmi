var hmiPopupWindows = [];

function navigateToDiagram(diagramId, target) {

    if (hmiPopupWindows != null)
    {          
        for(let i = 0; i < hmiPopupWindows.length; ++i) {
            try {
                const w = hmiPopupWindows[i];
                if (w.location.href.endsWith(diagramId)) {                    
                    if (!w.closed) {                                        
                        w.focus();
                        return;
                    }
                    else {
                        hmiPopupWindows.splice(i, 1);                    
                    }
                }
            } catch (e) {}
        }                
    }

    const w = window.open('/hmi?id=' + diagramId, target, 'toolbar=0,width=750,height=700', true);
    hmiPopupWindows.push(w);
}

function navigateToExternalLink(url, target) {
    window.open(url, target);
}