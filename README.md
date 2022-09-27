# ION-Update (Rust)
### [ION-Update](https://github.com/jimurrito/ION-Update) rewritten into Rust
Keeps IONOS DNS A Records up to date with your current public ip.

## Overview

Docker Conatiner built to keep IONOS DNS records up-to-date. Works off of a list of DNS Zones, all within the same IONOS tenant. Current Public IP is automatically pulled from the internet. IP Address is updated on container boot, and every 4hrs after.

**Recommended to validate the IONOS provided keys, prior to using them in the conatianer.**

The purpose of this container is to allow you to utilize a dynamic Public IP, as a static IP. Keeping DNS records updated with the correct IP to get back to your lab.

### Requirements
- a Wildcard 'A' Record on the target DNS Zone
- IONOS Developer Public and Private Key. **Note**: You must contact IONOS support to initally enable the feature. [IONOS Docs](https://developer.hosting.ionos.es/docs/getstarted).
- a string of DNS Zones, seperated by a comma.

## Docker Configuration

Example Docker Run
``` docker
docker run -it \
    # Required
    -e SCOPE="domain1.com,domain2.com" \
    -e PUBKEY="<PublicKey>" \
    -e PRVKEY="<PrivateKey>" \
    # Optional Tag - Default: Info
    -e LOG_LEVEL="<info,debug>" \
    jimurrito/ionupdate_rs:latest
```

### Links
- [GitHub Repo](https://github.com/jimurrito/ION-Update_rs)
- [Docker Repo](https://hub.docker.com/r/jimurrito/ionupdate_rs)

## FAQ

> **Q). Can I set Custom Public IP?** 
>
> A). No. Currently IP Address is only pulled from public endpoints.

> **Q). How do I debug Error "[1x2] Failed to Deserialize json into struct" ?**
>
> A). This error is generic, and typically happens when authentication or authorization to IONOS has failed. Please rerun the container with the Enviromental Variable LOG_LEVEL="debug". Test the API manually in apps like Postman can show more information as well. If you get any response other then "HTTP 200", please contact IONOS.

> **Q). Can I keep a sub-domain or non-wildcard record updated?**
>
> A). No. The architecture utilizes one wildcard record per DNS Zone.

For any other questions, please dont hesitate to reach out to me, via the [Issues tab on github.](https://github.com/jimurrito/ION-Update_rs/issues)
