<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET_user</name>
   <tag></tag>
   <elementGuidId>9b4ca780-0e41-4738-9ea4-7b0641c01533</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>10.3.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${GlobalVariable.URL}/users/${GlobalVariable.userID}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject&#xd;
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS&#xd;
import groovy.json.JsonSlurper&#xd;
import com.kms.katalon.core.testobject.ResponseObject&#xd;
import internal.GlobalVariable as GlobalVariable&#xd;
&#xd;
def response = WS.sendRequestAndVerify(findTestObject('Respon/GET_user'))&#xd;
&#xd;
assert GlobalVariable.userID != null : &quot;userID dari POST tidak ditemukan&quot;</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
