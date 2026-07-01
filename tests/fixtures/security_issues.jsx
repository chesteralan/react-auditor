import React from 'react';

function Profile({ userContent }) {
  return (
    <div dangerouslySetInnerHTML={{ __html: userContent }} />
  );
}

function unsafeProtocol() {
  const url = 'http://example.com';
  window.location.href = url;
}

export default Profile;
