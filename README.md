# sbb-data-downloader

This is a Cloudflare worker that allows downloading the latest version of a resource
from [opentransportdata.swiss](https://opentransportdata.swiss), which is useful, because SBB does not provide a static download link but a link changing with every update (which can be as often as daily).

## Endpoint
```http request
GET https://sbb-data-download.trainy.app/packages/:package_id/:file_name
```

Packages can be found on [opentransportdata.swiss](https://opentransportdata.swiss/).
For example, the package ID for the business organisation dataset is [`business-organisation-v2`](https://data.opentransportdata.swiss/dataset/business-organisation-v2), and the file name for the full business organisation CSV is `full-business-organisation.csv`.
