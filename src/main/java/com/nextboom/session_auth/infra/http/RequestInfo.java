package com.nextboom.session_auth.infra.http;

public class RequestInfo {
  private String ip;
  private String userAgent;

  public RequestInfo() {
  }

  public RequestInfo(String ip, String userAgent) {
    this.ip = ip;
    this.userAgent = userAgent;
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
}
