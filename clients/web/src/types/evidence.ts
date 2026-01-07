// Evidence/Occurrence types
export type EvidenceStatus = 'active' | 'exported' | 'archived' | 'deleted';

export interface Evidence {
  id: string;
  caseNumber: string;
  title: string;
  description: string;
  status: EvidenceStatus;
  priority: 'critical' | 'high' | 'medium' | 'low';
  createdBy: string;
  createdAt: Date;
  updatedAt: Date;
  attachments: EvidenceAttachment[];
  custodyChain: CustodyEntry[];
  relatedEvents: string[];
  tags: string[];
}

export interface EvidenceAttachment {
  id: string;
  filename: string;
  mimeType: string;
  size: number;
  url: string;
  uploadedBy: string;
  uploadedAt: Date;
  hash: string; // SHA-256 for integrity
}

export interface CustodyEntry {
  id: string;
  action: 'created' | 'modified' | 'exported' | 'viewed' | 'shared' | 'archived' | 'restored';
  userId: string;
  userName: string;
  timestamp: Date;
  details?: string;
  ipAddress?: string;
}

export interface EvidenceExport {
  evidenceId: string;
  format: 'zip' | 'pdf';
  includeVideos: boolean;
  includeChainOfCustody: boolean;
  password?: string;
  exportedAt: Date;
  exportedBy: string;
  downloadUrl: string;
}
