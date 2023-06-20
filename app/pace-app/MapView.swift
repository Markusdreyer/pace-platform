import MapKit
import Starscream
import SwiftUI

struct MapView: View {
    @StateObject private var viewModel = MapViewModel()

    var body: some View {
        ZStack {
            Map(coordinateRegion: $viewModel.region, showsUserLocation: true)
                .ignoresSafeArea()
            
            Circle()
                .fill(viewModel.isConnected ? Color.green : Color.red)
                .frame(width: 20, height: 20)
                .overlay(
                    Circle()
                        .stroke(Color.white, lineWidth: 2)
                )
                .offset(x: 150, y: -350)
            VStack {
                Spacer()
                VStack {
                    HStack {
                        TextField("User ID", text: $viewModel.userId)
                            .autocapitalization(.none)
                            .autocorrectionDisabled()
                            .textFieldStyle(RoundedBorderTextFieldStyle())
                            .padding()
                        TextField("Race ID", text: $viewModel.raceId)
                            .autocapitalization(.none)
                            .autocorrectionDisabled()
                            .textInputAutocapitalization(.none)
                            .textFieldStyle(RoundedBorderTextFieldStyle())
                            .padding()
                    }
                    Button(action: {
                        if viewModel.isConnected {
                            viewModel.disconnectFromWebSocket()
                        } else {
                            viewModel.connectToWebSocket()
                        }
                    }) {
                        Text(viewModel.isConnected ? "Disconnect": "Connect")
                    }
                }
                .padding()
                .background(Color(.secondarySystemBackground))
            }
        }
        .onAppear {
            viewModel.checkIfLocationServicesIsEnabled()
        }
    }
}

struct MapView_Previews: PreviewProvider {
    static var previews: some View {
        MapView()
    }
}

struct Coordinates: Codable {
    let lat: Double
    let long: Double
}

struct LocationUpdate: Codable {
    let userId: String
    let timestamp: Int
    let coordinates: Coordinates
}

final class MapViewModel: NSObject, ObservableObject, CLLocationManagerDelegate, WebSocketDelegate {
    var locationManager: CLLocationManager?
    var socket: WebSocket?
    
    @Published var region = MKCoordinateRegion(center: CLLocationCoordinate2D(latitude: 59.912922, longitude: 10.741735),
                                                   span: MKCoordinateSpan(latitudeDelta: 0.005, longitudeDelta: 0.005))
    @Published var isConnected = false
    @Published var userId = ""
    @Published var raceId = ""
    
    func checkIfLocationServicesIsEnabled() {
        if CLLocationManager.locationServicesEnabled() {
            locationManager = CLLocationManager()
            locationManager?.desiredAccuracy = kCLLocationAccuracyBest
            locationManager!.delegate = self
        } else {
            print("Location capabilities are turned off")
        }
    }
    
    func connectToWebSocket() {
        guard let locationManager = locationManager else { return }
        let remoteUrl: String = "wss://websockets.fly.dev/race/"
        let localUrl: String = "ws://localhost:8080/race/"
    
        let request = URLRequest(url: URL(string: localUrl + raceId)!)
        socket = WebSocket(request: request)
        socket?.delegate = self
        socket?.connect()

        locationManager.startUpdatingLocation()
    }
    
    func disconnectFromWebSocket() {
        isConnected = false
        socket?.disconnect()
        locationManager?.stopUpdatingLocation()
    }
    
    private func checkLocationAuthorization() {
        guard let locationManager = locationManager else { return }
        
        switch locationManager.authorizationStatus {
            
        case .notDetermined:
            locationManager.requestAlwaysAuthorization()
        case .restricted:
            print("Location is restricted")
        case .denied:
            print("Location permission denied")
        case .authorizedAlways, .authorizedWhenInUse:
            region = MKCoordinateRegion(center: locationManager.location!.coordinate, span:   MKCoordinateSpan(latitudeDelta: 0.005, longitudeDelta: 0.005) )
        @unknown default:
            break
        }
        
        locationManager.startUpdatingLocation()
    }
    
    func locationManager(_ manager: CLLocationManager, didUpdateLocations locations: [CLLocation]) {
        let locValue:CLLocationCoordinate2D = manager.location!.coordinate
        
        if !isConnected {
            print("Not connected, skipping sending data")
            return
        }

        let lat = locValue.latitude
        let long = locValue.longitude
        print("New Coordinates:")
        print(lat)
        print(long)
        
        let timestamp = Int(Date().timeIntervalSince1970)
        let coordinates = Coordinates(lat: lat, long: long)
        let locationUpdate = LocationUpdate(userId: userId, timestamp: timestamp, coordinates: coordinates)
        let encoder = JSONEncoder()
        guard let data = try? encoder.encode(locationUpdate) else { return }
        socket?.write(data: data)
    }
    
    func locationManagerDidChangeAuthorization(_ manager: CLLocationManager) {
        checkLocationAuthorization()
    }
    
    func didReceive(event: Starscream.WebSocketEvent, client: Starscream.WebSocket) {
        switch event {
            case .connected(let headers):
                isConnected = true
                print("websocket is connected: \(headers)")
            case .disconnected(let reason, let code):
                isConnected = false
                print("websocket is disconnected: \(reason) with code: \(code)")
            case .text(let string):
                print("Received text: \(string)")
            case .binary(let data):
                print("Received data: \(data.count)")
            case .ping(_):
                break
            case .pong(_):
                break
            case .viabilityChanged(_):
                break
            case .reconnectSuggested(_):
                break
            case .cancelled:
                isConnected = false
            case .error(let error):
                isConnected = false
                print(error!)
            }
    }
}
