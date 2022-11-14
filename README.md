# 
## How to run exemple

First run the cluster : `docker-compose up -d es01 es02 es03 kibana` 

Create index mapping:

```
PUT sncf
{
  "mappings": {
    "dynamic": false,
    "properties": {
      "Fin_controle": {
        "type": "date"
      },
      "Gare": {
        "type": "keyword"
      },
      "Gare_Code_UIC": {
        "type": "keyword"
      },
      "Nombre_Non_Conformites": {
        "type": "long"
      },
      "Nombre_observations": {
        "type": "long"
      },
      "Taux_conformite": {
        "type": "float"
      },
      "Jour_semaine": {
        "type": "keyword"
      },
      "Hour_Day": {
        "type": "keyword"
      },
      "event": {
        "properties": {
          "original": {
            "type": "keyword"
          }
        }
      },
      "log": {
        "properties": {
          "file": {
            "properties": {
              "path": {
                "type": "keyword"
              }
            }
          }
        }
      }
    }
  },
    "settings": {
      "number_of_replicas": 0
    }
}
```

Create pipeline: 

```
PUT _ingest/pipeline/sncf-pipeline
{
  "description": "Add new fields to sncf",
  "processors": [
    {
      "script": {
        "source":"""
        
        //calcul du taux
        if (ctx['Nombre_Non_Conformites'] > 0 && ctx['Nombre_observations'] > 0)
          ctx['Taux_conformite'] = 100.0 * ctx['Nombre_Non_Conformites'] / ctx['Nombre_observations'];
        
        ZonedDateTime finControle = ZonedDateTime.parse(ctx['Fin_controle']);
        // Calcul de l'heure du jour
        ctx['Hour_Day'] = finControle.getHour(); 
        // Calcul du jour de la semaine
        ctx['Jour_semaine'] = finControle.dayOfWeekEnum.getDisplayName(TextStyle.FULL, Locale.ROOT); 
        """
      }
    }
    ],
    "on_failure": [
      {
        "set": {
          "field": "_index",
          "value": "bad_docs"
        }
      }
    ]
}
```


Index types: 
  - data_hot : Données disponibles
  - data_warm : donnée en read_only
  - data_cold : read_only et données plus lentes
  - data_freeze : index fermés et réouvert sur demande(requete)

## datastream 

Le rollover se fait tous les x temps, modifiable avec : 
```

```

Créer le data-stream à partir d'un template d'index.
Créer une regle de retention de données dans Index Lifecycle policies, puis l'associer au template.