import asyncio
import os
import time
import logging
from bleak import BleakScanner

logging.basicConfig(
      level=logging.INFO,
      format="[%(asctime)s] %(levelname)s %(message)s",
      datefmt="%H:%M:%S"
      )
log = logging.getLogger("findhub")

SCAN_SECONDS  = int(os.environ.get("SCAN_SECONDS", "10"))
SEEN_WINDOW   = int(os.environ.get("SEEN_WINDOW", "60"))
LOOP_INTERVAL = int(os.environ.get("LOOP_INTERVAL", "15"))

seen_key = 0.0
seen_wallet = 0.0

def detection_callback(device, advertisement_data):
    for uuid, data in advertisement_data.service_data.items():
        if "feaa" in uuid.lower():
            t = hex(data[0])
            eid = data[1:21].hex()
            flag = data[21:22].hex()
            log.info(f"  FMDN frame: addr={device.address} Type={t} EID={eid} FLAG={flag}")
            if "0x41" in t:
                global seen_key
                seen_key = time.time();
                log.info(f"  Key detected!")

            if "0x40" in t: 
                global seen_wallet
                seen_wallet = time.time();
                log.info(f"  Wallet detected!")


async def scan_cycle():
    log.info(f"Starting scan for {SCAN_SECONDS}s...")
    scanner = BleakScanner(detection_callback=detection_callback)
    await scanner.start()
    await asyncio.sleep(SCAN_SECONDS)
    await scanner.stop()
    log.info("Scan complete")
 
async def main():
    log.info(f"Find Hub scanner starting")
    log.info(f"SCAN_SECONDS={SCAN_SECONDS} SEEN_WINDOW={SEEN_WINDOW} LOOP_INTERVAL={LOOP_INTERVAL}")
    while True:
        try:
            await scan_cycle()
        except Exception as e:
            log.error(f"Scan error: {e}", exc_info=True)
      
        log.info(f"Sleeping {LOOP_INTERVAL}s...")
        await asyncio.sleep(LOOP_INTERVAL)

asyncio.run(main())
