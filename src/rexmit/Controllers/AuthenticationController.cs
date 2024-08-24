// Copyright (c) Balanced Solutions Software. All Rights Reserved. Licensed under the MIT license. See LICENSE in the project root for license information.

using System.Threading.Tasks;
using Microsoft.AspNetCore.Authentication;
using Microsoft.AspNetCore.Authentication.Cookies;
using Microsoft.AspNetCore.DataProtection;
using Microsoft.AspNetCore.Mvc;

namespace rexmit.Controllers;

[Route("[controller]/[action]")] // Microsoft.AspNetCore.Mvc.Route
public class AuthenticationController(IDataProtectionProvider provider) : ControllerBase
{
    public IDataProtectionProvider Provider { get; } = provider;

    [HttpGet]
    public IActionResult Login(string returnUrl = "/")
    {
        return Challenge(new AuthenticationProperties { RedirectUri = returnUrl }, "Discord");
    }

    [HttpGet]
    public async Task<IActionResult> Logout(string returnUrl = "/")
    {
        //This removes the cookie assigned to the user login.
        await HttpContext.SignOutAsync(CookieAuthenticationDefaults.AuthenticationScheme);
        return LocalRedirect(returnUrl);
    }
}
