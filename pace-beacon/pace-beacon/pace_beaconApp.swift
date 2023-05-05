//
//  pace_beaconApp.swift
//  pace-beacon
//
//  Created by Markus Dreyer on 04/05/2023.
//


//A super simple app that streams location data from the device to a websocket endpoint.

import SwiftUI

@main
struct pace_beaconApp: App {
    var body: some Scene {
        WindowGroup {
            MapView()
        }
    }
}

