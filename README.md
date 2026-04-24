Aether SOS is a low-power mesh networking communication system designed to operate in extreme disaster environments.
Its core relies on "The Unselfish Principle".

### ⚠️ Legal & Moral Disclaimer

**By joining the Aether network, you agree to become a part of someone else's lifeline.**

While running in the background, this software will automatically and silently relay SOS Packets from other trapped individuals.
1. **Mandatory Relay**: When your device's battery is above 10%, the system will automatically consume a minuscule amount of power and bandwidth to help strangers nearby transmit their SOS signals (a TTL limit mechanism ensures no network storms are triggered).
2. **Gateway Node Dispatch**: The relay continues until the packet reaches **any phone that has successfully connected to a "Cell Tower" or "Low Earth Orbit Satellite (SpaceX Starlink / D2D)"**. That phone instantly transforms into a "Gateway Node", dispatching the collected SOS packets directly to national emergency dispatch centers (fire stations and hospitals) via the global communication network.
3. **Privacy Guarantee**: As a relay node, you (and your device) are **absolutely unable to decrypt** the forwarded message content (`Encrypted_Payload`). The relay only handles transmission; the communication is fully end-to-end encrypted (ChaCha20-Poly1305).
4. **Disclaimer**: Under extreme disaster conditions, this system provides communication assistance on a best-effort basis. The development team does not guarantee a 100% transmission success rate and assumes no legal liability for any loss of life or property arising from the use of this system.

### 📡 Technical Deep Dive: Packet Transmission Principles

To achieve the extreme requirement of communicating in airplane mode, without a cellular network, or even **with the SIM card removed**, Aether SOS abandons traditional handshake connections and adopts the following packet transmission principles:

1. **Connectionless Broadcast via NAN SDI**
   - **Completely SIM-less**: As long as the phone's hardware Wi-Fi chip is intact, even without a SIM card or in airplane mode, it can participate in the rescue network.
   - Traditional Wi-Fi Direct or Bluetooth requires pairing and connection establishment (taking about 3~5 seconds), which is impractical in disasters.
   - Aether utilizes the **Service Discovery Indicator (SDI)** mechanism of Wi-Fi NAN (Neighbor Awareness Networking) and Apple's AWDL Bonjour TXT Records.
   - Packet data is extremely compressed to **under 256 Bytes** and stuffed directly into the low-level "Discovery Frame" of the Wi-Fi chip.
   - As long as the phone emits a signal to discover nodes, the packet is piggybacked and delivered to all devices within a 50-meter radius, achieving true "instant broadcast."

2. **Smart Anti-Storm Routing**
   - **LRU Cache De-duplication**: Each node maintains an LRU cache of the last 5 minutes. If a packet with the same `message_id` is received, it is gracefully discarded to prevent packets from infinitely bouncing between two phones.
   - **TTL Hop Control**: Packets have a built-in Time-to-Live (default 7 hops). For every phone it passes through, the TTL decreases by 1. When the TTL reaches zero, the packet rests in peace, ensuring that the city's network bandwidth is not paralyzed.

3. **The Last Mile: Global Gateway Dispatch**
   - Packets expand outward through the crowd like a ripple until they touch a "Wide Area Network".
   - When any participating phone detects it is connected to a **Cell Tower** or a **Low Earth Orbit Satellite (e.g., SpaceX Direct-to-Cell / NTN)**, it automatically upgrades itself to a **"Global Gateway Node"**.
   - To ensure absolute delivery in an extremely congested disaster network, Aether **does not rely on the standard Internet (HTTP APIs)**, but directly utilizes low-level communication channels:
     - **Satellite Direct-to-Cell (NTN)**: Even if terrestrial base stations are completely destroyed, as long as there is an unobstructed view of the sky, the phone will shoot the packet to LEO satellite networks like SpaceX / Globalstar, achieving global rescue coverage ignoring terrain.
     - **SIM-less Emergency Bearer Services (EBS)**: Just like dialing 112/911 without a SIM card, Aether attempts to invoke low-level Emergency Bearer Services. As long as the phone detects a weak signal from ANY carrier, even without a SIM card or a monthly plan, it forces a connection.
     - **Stealth Data SMS / AML**: If the device has a SIM card, the SOS packet is extremely compressed and converted into a silent binary SMS, sent to national emergency shortcodes (like 911 / 112 backends). A single bar of voice signal is enough to penetrate.
     - **QoS Priority Flag**: Utilizes the low-level `TelephonyManager` to mark the transmission packet with the highest priority, forcing congested cell towers to yield.
   - Once received by the telecom facility or satellite ground station, it connects directly via fiber backbone to the National Disaster Management Center (NDMCC) and the nearest fire station/hospital for emergency dispatch, achieving true decentralized and high-survivability communication.

By using this system, you understand and agree to the principles above. In our darkest hours, let us be the breaking dawn for one another.
