//
//  mc.h
//  mc
//
//  Created by Yury Korolev on 1/18/24.
//

#import <Foundation/Foundation.h>
#import <MultipeerConnectivity/MultipeerConnectivity.h>

NS_ASSUME_NONNULL_BEGIN

Class MC_PEER_ID;
Class MC_NEARBY_SERVICE_ADVERTISER;
Class MC_NEARBY_SERVICE_BROWSER;
Class MC_SESSION;
Class MC_ADVERTISER_ASSISTANT;
Class MC_BROWSER_VIEW_CONTROLLER;

__attribute__((constructor))
static void mc_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        
        MC_PEER_ID = [MCPeerID class];
        MC_NEARBY_SERVICE_ADVERTISER = [MCNearbyServiceAdvertiser class];
        MC_NEARBY_SERVICE_BROWSER = [MCNearbyServiceBrowser class];
        MC_SESSION = [MCSession class];
        MC_ADVERTISER_ASSISTANT = [MCAdvertiserAssistant class];
        MC_BROWSER_VIEW_CONTROLLER = [MCBrowserViewController class];
    }
}

NS_ASSUME_NONNULL_END
