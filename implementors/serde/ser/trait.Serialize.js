(function() {var implementors = {};
implementors["twilight_http"] = [{"text":"impl Serialize for ErrorCode","synthetic":false,"types":[]},{"text":"impl Serialize for ApiError","synthetic":false,"types":[]},{"text":"impl Serialize for GeneralApiError","synthetic":false,"types":[]},{"text":"impl Serialize for MessageApiError","synthetic":false,"types":[]},{"text":"impl Serialize for MessageApiErrorEmbedField","synthetic":false,"types":[]},{"text":"impl Serialize for RatelimitedApiError","synthetic":false,"types":[]},{"text":"impl Serialize for ParseTypes","synthetic":false,"types":[]},{"text":"impl Serialize for AllowedMentions","synthetic":false,"types":[]}];
implementors["twilight_lavalink"] = [{"text":"impl Serialize for Opcode","synthetic":false,"types":[]},{"text":"impl Serialize for OutgoingEvent","synthetic":false,"types":[]},{"text":"impl Serialize for Destroy","synthetic":false,"types":[]},{"text":"impl Serialize for Equalizer","synthetic":false,"types":[]},{"text":"impl Serialize for EqualizerBand","synthetic":false,"types":[]},{"text":"impl Serialize for Pause","synthetic":false,"types":[]},{"text":"impl Serialize for Play","synthetic":false,"types":[]},{"text":"impl Serialize for Seek","synthetic":false,"types":[]},{"text":"impl Serialize for Stop","synthetic":false,"types":[]},{"text":"impl Serialize for VoiceUpdate","synthetic":false,"types":[]},{"text":"impl Serialize for SlimVoiceServerUpdate","synthetic":false,"types":[]},{"text":"impl Serialize for Volume","synthetic":false,"types":[]},{"text":"impl Serialize for IncomingEvent","synthetic":false,"types":[]},{"text":"impl Serialize for PlayerUpdate","synthetic":false,"types":[]},{"text":"impl Serialize for PlayerUpdateState","synthetic":false,"types":[]},{"text":"impl Serialize for Stats","synthetic":false,"types":[]},{"text":"impl Serialize for StatsCpu","synthetic":false,"types":[]},{"text":"impl Serialize for StatsFrames","synthetic":false,"types":[]},{"text":"impl Serialize for StatsMemory","synthetic":false,"types":[]},{"text":"impl Serialize for TrackEventType","synthetic":false,"types":[]},{"text":"impl Serialize for TrackEnd","synthetic":false,"types":[]},{"text":"impl Serialize for TrackStart","synthetic":false,"types":[]},{"text":"impl Serialize for LoadType","synthetic":false,"types":[]},{"text":"impl Serialize for Track","synthetic":false,"types":[]},{"text":"impl Serialize for TrackInfo","synthetic":false,"types":[]},{"text":"impl Serialize for PlaylistInfo","synthetic":false,"types":[]},{"text":"impl Serialize for LoadedTracks","synthetic":false,"types":[]},{"text":"impl Serialize for FailingAddress","synthetic":false,"types":[]},{"text":"impl Serialize for IpBlockType","synthetic":false,"types":[]},{"text":"impl Serialize for IpBlock","synthetic":false,"types":[]},{"text":"impl Serialize for RoutePlannerType","synthetic":false,"types":[]},{"text":"impl Serialize for RoutePlanner","synthetic":false,"types":[]},{"text":"impl Serialize for NanoIpRoutePlanner","synthetic":false,"types":[]},{"text":"impl Serialize for NanoIpDetails","synthetic":false,"types":[]},{"text":"impl Serialize for RotatingIpRoutePlanner","synthetic":false,"types":[]},{"text":"impl Serialize for RotatingIpDetails","synthetic":false,"types":[]},{"text":"impl Serialize for RotatingNanoIpRoutePlanner","synthetic":false,"types":[]},{"text":"impl Serialize for RotatingNanoIpDetails","synthetic":false,"types":[]}];
implementors["twilight_model"] = [{"text":"impl Serialize for EmbedAuthor","synthetic":false,"types":[]},{"text":"impl Serialize for EmbedField","synthetic":false,"types":[]},{"text":"impl Serialize for EmbedFooter","synthetic":false,"types":[]},{"text":"impl Serialize for EmbedImage","synthetic":false,"types":[]},{"text":"impl Serialize for EmbedProvider","synthetic":false,"types":[]},{"text":"impl Serialize for EmbedThumbnail","synthetic":false,"types":[]},{"text":"impl Serialize for EmbedVideo","synthetic":false,"types":[]},{"text":"impl Serialize for Embed","synthetic":false,"types":[]},{"text":"impl Serialize for MessageActivity","synthetic":false,"types":[]},{"text":"impl Serialize for MessageActivityType","synthetic":false,"types":[]},{"text":"impl Serialize for MessageApplication","synthetic":false,"types":[]},{"text":"impl Serialize for MessageFlags","synthetic":false,"types":[]},{"text":"impl Serialize for MessageType","synthetic":false,"types":[]},{"text":"impl Serialize for MessageReaction","synthetic":false,"types":[]},{"text":"impl Serialize for MessageReference","synthetic":false,"types":[]},{"text":"impl Serialize for Message","synthetic":false,"types":[]},{"text":"impl Serialize for PermissionOverwrite","synthetic":false,"types":[]},{"text":"impl Serialize for Attachment","synthetic":false,"types":[]},{"text":"impl Serialize for CategoryChannel","synthetic":false,"types":[]},{"text":"impl Serialize for ChannelMention","synthetic":false,"types":[]},{"text":"impl Serialize for ChannelType","synthetic":false,"types":[]},{"text":"impl Serialize for Group","synthetic":false,"types":[]},{"text":"impl Serialize for PrivateChannel","synthetic":false,"types":[]},{"text":"impl Serialize for Reaction","synthetic":false,"types":[]},{"text":"impl Serialize for ReactionType","synthetic":false,"types":[]},{"text":"impl Serialize for TextChannel","synthetic":false,"types":[]},{"text":"impl Serialize for VoiceChannel","synthetic":false,"types":[]},{"text":"impl Serialize for Webhook","synthetic":false,"types":[]},{"text":"impl Serialize for WebhookType","synthetic":false,"types":[]},{"text":"impl Serialize for Channel","synthetic":false,"types":[]},{"text":"impl Serialize for GuildChannel","synthetic":false,"types":[]},{"text":"impl Serialize for BotConnectionInfo","synthetic":false,"types":[]},{"text":"impl Serialize for ConnectionInfo","synthetic":false,"types":[]},{"text":"impl Serialize for GatewayEvent","synthetic":false,"types":[]},{"text":"impl Serialize for Connected","synthetic":false,"types":[]},{"text":"impl Serialize for Connecting","synthetic":false,"types":[]},{"text":"impl Serialize for Disconnected","synthetic":false,"types":[]},{"text":"impl Serialize for Identifying","synthetic":false,"types":[]},{"text":"impl Serialize for Payload","synthetic":false,"types":[]},{"text":"impl Serialize for Reconnecting","synthetic":false,"types":[]},{"text":"impl Serialize for Resuming","synthetic":false,"types":[]},{"text":"impl Serialize for ShardEvent","synthetic":false,"types":[]},{"text":"impl Serialize for DispatchEvent","synthetic":false,"types":[]},{"text":"impl Serialize for EventType","synthetic":false,"types":[]},{"text":"impl Serialize for Identify","synthetic":false,"types":[]},{"text":"impl Serialize for IdentifyInfo","synthetic":false,"types":[]},{"text":"impl Serialize for IdentifyProperties","synthetic":false,"types":[]},{"text":"impl Serialize for ReactionRemoveEmoji","synthetic":false,"types":[]},{"text":"impl Serialize for PartialEmoji","synthetic":false,"types":[]},{"text":"impl Serialize for Resume","synthetic":false,"types":[]},{"text":"impl Serialize for ResumeInfo","synthetic":false,"types":[]},{"text":"impl Serialize for UpdateStatus","synthetic":false,"types":[]},{"text":"impl Serialize for UpdateStatusInfo","synthetic":false,"types":[]},{"text":"impl Serialize for BanAdd","synthetic":false,"types":[]},{"text":"impl Serialize for BanRemove","synthetic":false,"types":[]},{"text":"impl Serialize for ChannelCreate","synthetic":false,"types":[]},{"text":"impl Serialize for ChannelDelete","synthetic":false,"types":[]},{"text":"impl Serialize for ChannelPinsUpdate","synthetic":false,"types":[]},{"text":"impl Serialize for ChannelUpdate","synthetic":false,"types":[]},{"text":"impl Serialize for GuildCreate","synthetic":false,"types":[]},{"text":"impl Serialize for GuildDelete","synthetic":false,"types":[]},{"text":"impl Serialize for GuildEmojisUpdate","synthetic":false,"types":[]},{"text":"impl Serialize for GuildIntegrationsUpdate","synthetic":false,"types":[]},{"text":"impl Serialize for GuildUpdate","synthetic":false,"types":[]},{"text":"impl Serialize for Heartbeat","synthetic":false,"types":[]},{"text":"impl Serialize for InviteCreate","synthetic":false,"types":[]},{"text":"impl Serialize for InviteDelete","synthetic":false,"types":[]},{"text":"impl Serialize for MemberAdd","synthetic":false,"types":[]},{"text":"impl Serialize for MemberChunk","synthetic":false,"types":[]},{"text":"impl Serialize for MemberRemove","synthetic":false,"types":[]},{"text":"impl Serialize for MemberUpdate","synthetic":false,"types":[]},{"text":"impl Serialize for MessageCreate","synthetic":false,"types":[]},{"text":"impl Serialize for MessageDelete","synthetic":false,"types":[]},{"text":"impl Serialize for MessageDeleteBulk","synthetic":false,"types":[]},{"text":"impl Serialize for MessageUpdate","synthetic":false,"types":[]},{"text":"impl Serialize for PresenceUpdate","synthetic":false,"types":[]},{"text":"impl Serialize for ReactionAdd","synthetic":false,"types":[]},{"text":"impl Serialize for ReactionRemove","synthetic":false,"types":[]},{"text":"impl Serialize for ReactionRemoveAll","synthetic":false,"types":[]},{"text":"impl Serialize for Ready","synthetic":false,"types":[]},{"text":"impl Serialize for RequestGuildMembers","synthetic":false,"types":[]},{"text":"impl Serialize for RoleCreate","synthetic":false,"types":[]},{"text":"impl Serialize for RoleDelete","synthetic":false,"types":[]},{"text":"impl Serialize for RoleUpdate","synthetic":false,"types":[]},{"text":"impl Serialize for TypingStart","synthetic":false,"types":[]},{"text":"impl Serialize for UnavailableGuild","synthetic":false,"types":[]},{"text":"impl Serialize for UpdateVoiceState","synthetic":false,"types":[]},{"text":"impl Serialize for UserUpdate","synthetic":false,"types":[]},{"text":"impl Serialize for VoiceServerUpdate","synthetic":false,"types":[]},{"text":"impl Serialize for VoiceStateUpdate","synthetic":false,"types":[]},{"text":"impl Serialize for WebhooksUpdate","synthetic":false,"types":[]},{"text":"impl Serialize for Activity","synthetic":false,"types":[]},{"text":"impl Serialize for ActivityAssets","synthetic":false,"types":[]},{"text":"impl Serialize for ActivityEmoji","synthetic":false,"types":[]},{"text":"impl Serialize for ActivityFlags","synthetic":false,"types":[]},{"text":"impl Serialize for ActivityParty","synthetic":false,"types":[]},{"text":"impl Serialize for ActivitySecrets","synthetic":false,"types":[]},{"text":"impl Serialize for ActivityTimestamps","synthetic":false,"types":[]},{"text":"impl Serialize for ActivityType","synthetic":false,"types":[]},{"text":"impl Serialize for ClientStatus","synthetic":false,"types":[]},{"text":"impl Serialize for Status","synthetic":false,"types":[]},{"text":"impl Serialize for Presence","synthetic":false,"types":[]},{"text":"impl Serialize for UserOrId","synthetic":false,"types":[]},{"text":"impl Serialize for GatewayIntents","synthetic":false,"types":[]},{"text":"impl Serialize for OpCode","synthetic":false,"types":[]},{"text":"impl Serialize for SessionStartLimit","synthetic":false,"types":[]},{"text":"impl Serialize for AuditLogChange","synthetic":false,"types":[]},{"text":"impl Serialize for AuditLogChangeKey","synthetic":false,"types":[]},{"text":"impl Serialize for AuditLogEntry","synthetic":false,"types":[]},{"text":"impl Serialize for AuditLogEvent","synthetic":false,"types":[]},{"text":"impl Serialize for AuditLogOptionalEntryInfo","synthetic":false,"types":[]},{"text":"impl Serialize for PartialGuildIntegration","synthetic":false,"types":[]},{"text":"impl Serialize for AuditLog","synthetic":false,"types":[]},{"text":"impl Serialize for Member","synthetic":false,"types":[]},{"text":"impl Serialize for Ban","synthetic":false,"types":[]},{"text":"impl Serialize for DefaultMessageNotificationLevel","synthetic":false,"types":[]},{"text":"impl Serialize for Emoji","synthetic":false,"types":[]},{"text":"impl Serialize for ExplicitContentFilter","synthetic":false,"types":[]},{"text":"impl Serialize for GuildInfo","synthetic":false,"types":[]},{"text":"impl Serialize for GuildIntegration","synthetic":false,"types":[]},{"text":"impl Serialize for IntegrationAccount","synthetic":false,"types":[]},{"text":"impl Serialize for MfaLevel","synthetic":false,"types":[]},{"text":"impl Serialize for PartialGuild","synthetic":false,"types":[]},{"text":"impl Serialize for PartialMember","synthetic":false,"types":[]},{"text":"impl Serialize for Permissions","synthetic":false,"types":[]},{"text":"impl Serialize for PremiumTier","synthetic":false,"types":[]},{"text":"impl Serialize for GuildPreview","synthetic":false,"types":[]},{"text":"impl Serialize for GuildPrune","synthetic":false,"types":[]},{"text":"impl Serialize for Role","synthetic":false,"types":[]},{"text":"impl Serialize for GuildStatus","synthetic":false,"types":[]},{"text":"impl Serialize for SystemChannelFlags","synthetic":false,"types":[]},{"text":"impl Serialize for UnavailableGuild","synthetic":false,"types":[]},{"text":"impl Serialize for VerificationLevel","synthetic":false,"types":[]},{"text":"impl Serialize for GuildWidget","synthetic":false,"types":[]},{"text":"impl Serialize for Guild","synthetic":false,"types":[]},{"text":"impl Serialize for ApplicationId","synthetic":false,"types":[]},{"text":"impl Serialize for AttachmentId","synthetic":false,"types":[]},{"text":"impl Serialize for AuditLogEntryId","synthetic":false,"types":[]},{"text":"impl Serialize for ChannelId","synthetic":false,"types":[]},{"text":"impl Serialize for EmojiId","synthetic":false,"types":[]},{"text":"impl Serialize for GenericId","synthetic":false,"types":[]},{"text":"impl Serialize for GuildId","synthetic":false,"types":[]},{"text":"impl Serialize for IntegrationId","synthetic":false,"types":[]},{"text":"impl Serialize for MessageId","synthetic":false,"types":[]},{"text":"impl Serialize for RoleId","synthetic":false,"types":[]},{"text":"impl Serialize for UserId","synthetic":false,"types":[]},{"text":"impl Serialize for WebhookId","synthetic":false,"types":[]},{"text":"impl Serialize for InviteMetadata","synthetic":false,"types":[]},{"text":"impl Serialize for TargetUserType","synthetic":false,"types":[]},{"text":"impl Serialize for Invite","synthetic":false,"types":[]},{"text":"impl Serialize for SkuId","synthetic":false,"types":[]},{"text":"impl Serialize for TeamId","synthetic":false,"types":[]},{"text":"impl Serialize for TeamMember","synthetic":false,"types":[]},{"text":"impl Serialize for TeamMembershipState","synthetic":false,"types":[]},{"text":"impl Serialize for Team","synthetic":false,"types":[]},{"text":"impl Serialize for CurrentApplicationInfo","synthetic":false,"types":[]},{"text":"impl Serialize for Connection","synthetic":false,"types":[]},{"text":"impl Serialize for ConnectionVisibility","synthetic":false,"types":[]},{"text":"impl Serialize for CurrentUser","synthetic":false,"types":[]},{"text":"impl Serialize for UserFlags","synthetic":false,"types":[]},{"text":"impl Serialize for PremiumType","synthetic":false,"types":[]},{"text":"impl Serialize for UserProfile","synthetic":false,"types":[]},{"text":"impl Serialize for User","synthetic":false,"types":[]},{"text":"impl Serialize for VoiceState","synthetic":false,"types":[]},{"text":"impl Serialize for VoiceRegion","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()