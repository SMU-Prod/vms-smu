# Plano de Implementação - Sistema Completo
**Data**: 13/12/2025

---

## ESTRUTURA DE NOVOS SERVIÇOS

```
vms/
├── services/
│   ├── vms-ingest/          ✅ Existe (precisa ONVIF)
│   ├── vms-storage/         ✅ Existe
│   ├── vms-ai/              ✅ Existe (precisa expandir)
│   ├── vms-stream/          ✅ Existe
│   ├── vms-api/             ✅ Existe
│   ├── vms-gateway/         ✅ Existe
│   ├── vms-replicator/      ✅ Existe
│   │
│   ├── vms-events/          ❌ CRIAR - Sistema de eventos e alarmes
│   ├── vms-evidence/        ❌ CRIAR - Gerenciamento de ocorrências
│   ├── vms-analytics/       ❌ CRIAR - Análise de vídeo avançada
│   ├── vms-face/            ❌ CRIAR - Reconhecimento facial
│   ├── vms-lpr/             ❌ CRIAR - Leitura de placas
│   ├── vms-auth/            ❌ CRIAR - Autenticação e autorização
│   └── vms-notifications/   ❌ CRIAR - Notificações (email/SMS/push)
│
├── clients/
│   ├── web/                 ❌ CRIAR - Cliente SolidJS
│   ├── desktop/             ❌ CRIAR - Cliente Tauri
│   └── mobile/              ❌ CRIAR - Cliente Flutter
```

---

## ORDEM DE IMPLEMENTAÇÃO

### Sprint 1: ONVIF + Eventos (Esta semana)
**Tempo**: 5-7 dias

1. **vms-ingest: ONVIF Discovery**
   - WS-Discovery protocol
   - Autenticação ONVIF
   - GetProfiles / GetStreamUri
   - PTZ control básico

2. **vms-events: Sistema de Eventos**
   - Event broker (NATS streams)
   - Gerenciamento de alarmes
   - Motor de regras básico
   - API REST para alarmes

3. **vms-auth: Autenticação**
   - JWT tokens
   - RBAC básico
   - Usuários e permissões

### Sprint 2: Evidence + Cliente Web (Semana 2)
**Tempo**: 7-10 dias

4. **vms-evidence: Ocorrências**
   - CRUD de ocorrências
   - Anexos (vídeo/imagem/doc)
   - Workflow (status, responsável)
   - Relatórios básicos

5. **clients/web: Cliente SolidJS**
   - Live view (grid de câmeras)
   - WebRTC player
   - Timeline de playback
   - Painel de alarmes
   - Gerenciamento de câmeras

### Sprint 3: Análise de Vídeo (Semana 3-4)
**Tempo**: 10-14 dias

6. **vms-analytics: VA Avançado**
   - Motion detection
   - Linhas virtuais
   - Áreas virtuais
   - Objeto abandonado/removido
   - Integração com vms-events

### Sprint 4: Reconhecimento Facial (Semana 5-6)
**Tempo**: 10-14 dias

7. **vms-face: Face Recognition**
   - Detector MTCNN/RetinaFace
   - Embedding ArcFace
   - Banco de faces
   - Matching engine
   - API de cadastro

### Sprint 5: LPR (Semana 7-8)
**Tempo**: 10-14 dias

8. **vms-lpr: Leitura de Placas**
   - Detecção de veículo
   - Detecção de placa
   - OCR (PaddleOCR)
   - Banco de placas
   - Lista branca/negra

### Sprint 6: Notificações (Semana 9)
**Tempo**: 5-7 dias

9. **vms-notifications: Notificações**
   - Email (SMTP)
   - SMS (Twilio/Nexmo)
   - Push (FCM)
   - Webhooks

### Sprint 7: Cliente Desktop (Semana 10-11)
**Tempo**: 10-14 dias

10. **clients/desktop: Tauri**
    - Live view com grid
    - Decodificação hardware
    - PTZ control
    - Playback sincronizado

### Sprint 8: Mobile (Semana 12-13)
**Tempo**: 10-14 dias

11. **clients/mobile: Flutter**
    - Live view
    - Push notifications
    - Alarmes
    - Playback básico

---

## COMEÇANDO AGORA

Vou implementar na seguinte ordem:

1. ✅ **ONVIF module** no vms-ingest
2. ✅ **vms-events** service
3. ✅ **vms-auth** service
4. ✅ **vms-evidence** service
5. ✅ **Cliente Web** básico

Iniciando Sprint 1 agora!
