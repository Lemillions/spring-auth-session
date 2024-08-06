package com.nextboom.session_auth.infra.redis;

import java.time.LocalDateTime;
import java.util.LinkedHashMap;
import java.util.Map;

import com.nextboom.session_auth.infra.http.RequestInfo;

public class Session {
  public static final int EXPIRE_TIME = 60 * 60 * 24 * 7; // 7 days
  public static final String DATA_PREFIX = "session_data:";
  private String token;
  private String userId;
  private String createdAt;
  private String expiresAt;
  private String lastAccessAt;
  private String ip;
  private String userAgent;

  public Session() {
  }

  public Session(String token, String userId, String createdAt, String expiresAt, String lastAccessAt, String ip,
      String userAgent) {
    this.token = token;
    this.userId = userId;
    this.createdAt = createdAt;
    this.expiresAt = expiresAt;
    this.lastAccessAt = lastAccessAt;
    this.ip = ip;
    this.userAgent = userAgent;
  }

  public static Session create(String token, String userId, RequestInfo requestInfo) {
    LocalDateTime now = LocalDateTime.now();
    return new Session(
        token,
        userId,
        now.toString(),
        now.plusSeconds(EXPIRE_TIME).toString(),
        now.toString(),
        requestInfo.getIp(),
        requestInfo.getUserAgent());
  }

  public String getToken() {
    return token;
  }

  public void setToken(String token) {
    this.token = token;
  }

  public String getUserId() {
    return userId;
  }

  public void setUserId(String userId) {
    this.userId = userId;
  }

  public String getCreatedAt() {
    return createdAt;
  }

  public void setCreatedAt(String createdAt) {
    this.createdAt = createdAt;
  }

  public String getExpiresAt() {
    return expiresAt;
  }

  public void setExpiresAt(String expiresAt) {
    this.expiresAt = expiresAt;
  }

  public String getLastAccessAt() {
    return lastAccessAt;
  }

  public void setLastAccessAt(String lastAccessAt) {
    this.lastAccessAt = lastAccessAt;
  }

  public String getIp() {
    return ip;
  }

  public void setIp(String ip) {
    this.ip = ip;
  }

  public String getUserAgent() {
    return userAgent;
  }

  public void setUserAgent(String userAgent) {
    this.userAgent = userAgent;
  }

  public boolean isExpired() {
    return LocalDateTime.now().isAfter(LocalDateTime.parse(expiresAt));
  }

  public Map<String, String> toMap() {
    Map<String, String> map = new LinkedHashMap<>();
    map.put("token", token);
    map.put("userId", userId);
    map.put("createdAt", createdAt);
    map.put("expiresAt", expiresAt);
    map.put("lastAccessAt", lastAccessAt);
    map.put("ip", ip);
    map.put("userAgent", userAgent);
    return map;
  }

  @Override
  public String toString() {
    return "{" + "token: " + token + ", userId: " + userId + ", createdAt: " + createdAt + ", expiresAt: " + expiresAt
        + ", lastAccessAt: " + lastAccessAt + ", ip: " + ip + ", userAgent: " + userAgent + "}";
  }

  public void updateLastAccess() {
    lastAccessAt = LocalDateTime.now().toString();
  }
}
