/**
 * OpenAPI Petstore
 * This spec is mainly for testing Petstore server and contains fake endpoints, models. Please do not use this for any other purpose. Special characters: \" \\
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 *
 */

import ApiClient from '../ApiClient';

/**
 * The HealthCheckResult model module.
 * @module model/HealthCheckResult
 * @version 1.0.0
 */
class HealthCheckResult {
    /**
     * Constructs a new <code>HealthCheckResult</code>.
     * Just a string to inform instance is up and running. Make it nullable in hope to get it as pointer in generated model.
     * @alias module:model/HealthCheckResult
     */
    constructor() { 
        
        HealthCheckResult.initialize(this);
    }

    /**
     * Initializes the fields of this object.
     * This method is used by the constructors of any subclasses, in order to implement multiple inheritance (mix-ins).
     * Only for internal use.
     */
    static initialize(obj) { 
    }

    /**
     * Constructs a <code>HealthCheckResult</code> from a plain JavaScript object, optionally creating a new instance.
     * Copies all relevant properties from <code>data</code> to <code>obj</code> if supplied or a new instance if not.
     * @param {Object} data The plain JavaScript object bearing properties of interest.
     * @param {module:model/HealthCheckResult} obj Optional instance to populate.
     * @return {module:model/HealthCheckResult} The populated <code>HealthCheckResult</code> instance.
     */
    static constructFromObject(data, obj) {
        if (data) {
            obj = obj || new HealthCheckResult();

            if (data.hasOwnProperty('NullableMessage')) {
                obj['NullableMessage'] = ApiClient.convertToType(data['NullableMessage'], 'String');
            }
        }
        return obj;
    }


}

/**
 * @member {String} NullableMessage
 */
HealthCheckResult.prototype['NullableMessage'] = undefined;






export default HealthCheckResult;

